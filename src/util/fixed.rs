use num_traits::Zero;
use std::{
    fmt::{Binary, Debug, Display},
    ops::{Add, AddAssign, Mul, Neg, Shr, Sub, SubAssign},
};

const POS: bool = false;
const NEG: bool = true;

#[derive(Debug, Clone, PartialEq)]
pub struct FixedDec {
    sign: bool,
    dec: i32,
    parts: Vec<u32>,
}

impl FixedDec {
    pub fn zeros() -> Self {
        Self::zero()
    }

    pub fn dec_len(&self) -> i32 {
        self.parts.len() as i32 - self.dec
    }

    pub fn part(&self, i: i32) -> u32 {
        let Ok(i): Result<usize, _> = i.try_into() else {
            return self.pre_padding();
        };
        self.parts.get(i).cloned().unwrap_or(0)
    }

    pub fn is_pos(&self) -> bool {
        !self.sign
    }

    pub fn is_neg(&self) -> bool {
        self.sign
    }

    fn pre_padding(&self) -> u32 {
        match self.sign {
            POS => 0,
            NEG => !0,
        }
    }
}

impl Zero for FixedDec {
    fn zero() -> Self {
        Self {
            sign: POS,
            dec: 0,
            parts: Vec::new(),
        }
    }

    fn is_zero(&self) -> bool {
        self.parts.iter().all(|&b| b == 0)
    }
}

impl Shr<u32> for FixedDec {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut parts = Vec::new();
        let sr = rhs % 32;
        let sl = 32 - sr;
        let mask = (1 << sr) - 1;
        let dec = self.dec - (rhs / 32) as i32;
        let mut rem = 0;
        for part in self.parts {
            parts.push((part >> sr) ^ rem);
            rem = (part & mask) << sl;
        }
        if rem != 0 {
            parts.push(rem);
        }
        Self {
            dec,
            parts,
            sign: self.sign,
        }
    }
}

impl Add for FixedDec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&FixedDec> for FixedDec {
    fn add_assign(&mut self, rhs: &FixedDec) {
        let dec = self.dec.max(rhs.dec);
        let left_i = -dec;
        let right_i = self.dec_len().max(rhs.dec_len());
        let len = (right_i - left_i) as usize;
        if dec != self.dec {
            let fill = self.pre_padding();
            let fill_len = rhs.dec - self.dec;
            self.parts.splice(0..0, (0..fill_len).map(|_| fill));
            self.dec += fill_len;
        }
        if self.parts.len() != len {
            self.parts.resize(len, 0);
        }
        let mut carry = false;
        let rhs_offset = rhs.dec - self.dec;
        for i in (0..self.parts.len()).rev() {
            let a = self.parts[i];
            let b = rhs.part(i as i32 + rhs_offset);
            let (res, c) = a.carrying_add(b, carry);
            self.parts[i] = res;
            carry = c;
        }
        let sign = if self.sign == rhs.sign {
            if self.sign == POS && carry {
                self.parts.insert(0, 1);
                self.dec += 1;
            } else if self.sign == NEG && !carry {
                self.parts.insert(0, !1);
                self.dec += 1;
            }
            self.sign
        } else if carry {
            POS
        } else {
            NEG
        };
        self.sign = sign;
    }
}

impl SubAssign<&FixedDec> for FixedDec {
    fn sub_assign(&mut self, rhs: &FixedDec) {
        *self += &-rhs;
    }
}

impl Add for &FixedDec {
    type Output = FixedDec;

    fn add(self, rhs: Self) -> Self::Output {
        let mut dec = self.dec.max(rhs.dec);
        let left_i = -dec;
        let right_i = self.dec_len().max(rhs.dec_len());
        let mut parts = Vec::with_capacity((right_i - left_i) as usize);
        let mut carry = false;
        for i in (left_i..right_i).rev() {
            let a = self.part(i + self.dec);
            let b = rhs.part(i + rhs.dec);
            let (res, c) = a.carrying_add(b, carry);
            parts.push(res);
            carry = c;
        }
        let sign = if self.sign == rhs.sign {
            if self.is_pos() && carry {
                parts.push(1);
                dec += 1;
            } else if self.is_neg() && !carry {
                parts.push(!1);
                dec += 1;
            }
            self.sign
        } else if carry {
            POS
        } else {
            NEG
        };
        parts.reverse();
        FixedDec { parts, dec, sign }
    }
}

impl Sub for &FixedDec {
    type Output = FixedDec;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl Neg for &FixedDec {
    type Output = FixedDec;

    fn neg(self) -> Self::Output {
        let parts = self.parts.iter().map(|p| !p).collect();
        let mut res = FixedDec {
            parts,
            sign: !self.sign,
            dec: self.dec,
        };
        res += &Self::Output {
            parts: vec![1],
            dec: self.dec - (self.parts.len() as i32 - 1),
            sign: POS,
        };
        res
    }
}

impl Mul for &FixedDec {
    type Output = FixedDec;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<u32> = vec![0; self.parts.len() + rhs.parts.len()];
        for (i, &x) in self.parts.iter().enumerate().rev() {
            let mut carry: u32 = 0;
            for (j, &y) in rhs.parts.iter().enumerate().rev() {
                let (lsb, msb) = mul_lmsb(x, y);
                let k = i + j + 1;
                let (res, carry1) = parts[k].overflowing_add(lsb);
                parts[k] = res;
                let (res, carry2) = parts[k].overflowing_add(carry);
                parts[k] = res;
                // dude I have no clue if this can overflow; I know msb can take 1 without
                // overflowing, but I'm not sure if 2 can get here when it's max
                carry = (carry1 as u32) + (carry2 as u32) + msb;
            }
            if carry > 0 {
                parts[i] = carry;
            }
        }
        Self::Output {
            dec: self.dec + rhs.dec,
            parts,
            sign: self.sign == rhs.sign,
        }
    }
}

fn mul_lmsb(x: u32, y: u32) -> (u32, u32) {
    let lsb = x.wrapping_mul(y);
    let a = x & 0xffff;
    let b = x >> 16;
    let c = y & 0xffff;
    let d = y >> 16;
    let ad = a * d + ((a * c) >> 16);
    let bc = b * c;
    let car = ad > (0xffffffff - bc);
    let msb = ((ad + bc) >> 16) + ((car as u32) << 16) + b * d;
    (lsb, msb)
}

const INV_SIGN_MASK: u32 = (1 << 31) - 1;
const FRAC_BIT: u32 = 1 << 23;
const FRAC_MASK: u32 = FRAC_BIT - 1;

impl From<f32> for FixedDec {
    fn from(value: f32) -> Self {
        let raw = value.to_bits() & INV_SIGN_MASK;
        let exp = (raw >> 23) as i32 - 127;
        let frac = (raw & FRAC_MASK) + FRAC_BIT;
        let start = -exp - 1;
        let end = -exp + 23;
        let start_i = start.div_euclid(32);
        let end_i = (end - 1).div_euclid(32);
        let parts = if start_i == end_i {
            vec![frac << (8 - start.rem_euclid(32))]
        } else {
            let s = end.rem_euclid(32);
            vec![frac >> s, frac << (32 - s)]
        };
        Self {
            sign: POS,
            dec: -start_i,
            parts,
        }
    }
}

impl From<&FixedDec> for f32 {
    fn from(value: &FixedDec) -> Self {
        let mut sign = 0;
        let value = if value.is_neg() {
            sign = 1 << 31;
            &-value
        } else {
            value
        };
        let mut skip_count = 0;
        let mut iter = value
            .parts
            .iter()
            .inspect(|_| skip_count += 1)
            .skip_while(|&&x| x == 0);

        let Some(v) = iter.next() else {
            return 0.0;
        };
        let start = v.leading_zeros();
        let frac = if start > 9 {
            let sh = start - 9;
            (v << sh) + iter.next().copied().map(|v| v >> (32 - sh)).unwrap_or(0)
        } else {
            v >> (9 - start)
        };
        let exp = (127 - (skip_count * 32 + start)) << 23;
        let res = frac + exp + sign;
        println!();
        println!("res: {:032b}", res);
        println!("ans: {:032b}", 0.75f32.to_bits());
        f32::from_bits(res)
    }
}

impl Display for FixedDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f32::from(self))
    }
}

impl Binary for FixedDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign == NEG {
            write!(f, "-")?;
        }
        if self.dec < 0 {
            write!(f, ".")?;
            for _ in 0..(-self.dec) {
                write!(f, "00000000000000000000000000000000")?;
            }
        }
        for (i, part) in self.parts.iter().enumerate() {
            if i as i32 == self.dec {
                write!(f, ".")?;
            } else if i != 0 {
                write!(f, "_")?;
            }
            write!(f, "{:032b}", part)?;
        }
        Ok(())
    }
}
