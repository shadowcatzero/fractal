use super::*;

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign},
};

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

impl Shl<i32> for FixedDec {
    type Output = FixedDec;
    fn shl(self, rhs: i32) -> Self::Output {
        &self << rhs
    }
}

impl Shl<i32> for &FixedDec {
    type Output = FixedDec;
    fn shl(self, rhs: i32) -> Self::Output {
        self >> -rhs
    }
}

impl Shr<i32> for FixedDec {
    type Output = FixedDec;
    fn shr(self, rhs: i32) -> Self::Output {
        &self >> rhs
    }
}

impl Shr<i32> for &FixedDec {
    type Output = FixedDec;

    fn shr(self, rhs: i32) -> Self::Output {
        let mut parts = Vec::with_capacity(self.parts.len());
        let sr = rhs.rem_euclid(32);
        let sl = 32 - sr as u32;
        let mask = (1 << sr) - 1;
        let dec = self.dec - rhs.div_floor(32);
        let mut rem = 0;
        for part in &self.parts {
            parts.push((part >> sr) ^ rem);
            rem = (part & mask).unbounded_shl(sl);
        }
        if rem != 0 {
            parts.push(rem);
        }
        Self::Output {
            dec,
            parts,
            sign: self.sign,
        }
    }
}

impl Add for FixedDec {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += &rhs;
        self
    }
}

impl AddAssign<FixedDec> for FixedDec {
    fn add_assign(&mut self, rhs: FixedDec) {
        *self += &rhs
    }
}

impl AddAssign<&FixedDec> for FixedDec {
    fn add_assign(&mut self, rhs: &FixedDec) {
        let (dec, len) = new_dec(self, rhs);
        if dec != self.dec {
            let fill_len = rhs.dec - self.dec;
            self.parts.splice(0..0, (0..fill_len).map(|_| 0));
            self.dec += fill_len;
        }
        if self.parts.len() != len {
            self.parts.resize(len, 0);
        }
        let src = trust(&self.parts);
        add(self, &|i| src[i], rhs);
    }
}

pub fn trust<'b, T>(x: &T) -> &'b T {
    unsafe { std::mem::transmute(x) }
}

impl Add for &FixedDec {
    type Output = FixedDec;

    fn add(self, rhs: Self) -> Self::Output {
        let (dec, len) = new_dec(self, rhs);
        let mut parts = Vec::with_capacity(len);
        #[allow(clippy::uninit_vec)]
        unsafe {
            parts.set_len(len);
        }
        let mut res = FixedDec {
            sign: self.sign,
            dec,
            parts,
        };
        #[allow(clippy::suspicious_arithmetic_impl)]
        let offset = self.dec - dec;
        add(&mut res, &|i| self.part(i as i32 + offset), rhs);
        res
    }
}

fn add(dest: &mut FixedDec, src: &impl Fn(usize) -> u32, rhs: &FixedDec) {
    let mut carry = false;
    let rhs_offset = rhs.dec - dest.dec;
    if dest.sign == rhs.sign {
        for i in (0..dest.parts.len()).rev() {
            let a = src(i);
            let b = rhs.part(i as i32 + rhs_offset);
            (dest.parts[i], carry) = a.carrying_add(b, carry);
        }
        if carry {
            dest.parts.insert(0, 1);
            dest.dec += 1;
        }
    } else {
        for i in (0..dest.parts.len()).rev() {
            let a = src(i);
            let b = rhs.part(i as i32 + rhs_offset);
            (dest.parts[i], carry) = a.borrowing_sub(b, carry);
        }
        if carry {
            dest.sign = !dest.sign;
            for part in &mut dest.parts {
                *part = !*part;
            }
        }
    }
    dest.trim()
}

fn new_dec(x: &FixedDec, y: &FixedDec) -> (i32, usize) {
    let dec = x.dec.max(y.dec);
    let left_i = -dec;
    let right_i = x.dec_len().max(y.dec_len());
    let len = (right_i - left_i) as usize;
    (dec, len)
}

impl Sub for &FixedDec {
    type Output = FixedDec;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl Sub for FixedDec {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= &rhs;
        self
    }
}

impl SubAssign<&FixedDec> for FixedDec {
    fn sub_assign(&mut self, rhs: &FixedDec) {
        *self += &-rhs;
    }
}

impl SubAssign<FixedDec> for FixedDec {
    fn sub_assign(&mut self, rhs: FixedDec) {
        *self += &-rhs;
    }
}

impl Neg for &FixedDec {
    type Output = FixedDec;

    fn neg(self) -> Self::Output {
        let mut res = self.clone();
        res.sign = !res.sign;
        res
    }
}

impl Neg for FixedDec {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.sign = !self.sign;
        self
    }
}

impl Mul for &FixedDec {
    type Output = FixedDec;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign != rhs.sign;
        let mut parts: Vec<u32> = vec![0; self.parts.len() + rhs.parts.len()];
        let dec = self.dec + rhs.dec;

        for (i, &x) in self.parts.iter().enumerate().rev() {
            let mut carry: u32 = 0;
            for (j, &y) in rhs.parts.iter().enumerate().rev() {
                let (lsb, msb) = x.widening_mul(y);
                // let (lsb, msb) = mul_lmsb(x, y);
                let k = i + j + 1;
                let (res, carry1) = parts[k].overflowing_add(lsb);
                let (res, carry2) = res.overflowing_add(carry);
                parts[k] = res;
                // dude I have no clue if this can overflow; I know msb can take 1 without
                // overflowing, but I'm not sure if 2 can get here when it's max
                carry = (carry1 as u32) + (carry2 as u32) + msb;
            }
            parts[i] = carry
        }

        let mut res = Self::Output { dec, parts, sign };
        res.trim();
        res
    }
}

impl Mul for FixedDec {
    type Output = FixedDec;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&FixedDec> for FixedDec {
    type Output = FixedDec;

    fn mul(self, rhs: &FixedDec) -> Self::Output {
        &self * rhs
    }
}

impl Mul<FixedDec> for &FixedDec {
    type Output = FixedDec;

    fn mul(self, rhs: FixedDec) -> Self::Output {
        self * &rhs
    }
}

impl MulAssign<&FixedDec> for FixedDec {
    fn mul_assign(&mut self, rhs: &FixedDec) {
        *self = &*self * rhs
    }
}

impl MulAssign<FixedDec> for FixedDec {
    fn mul_assign(&mut self, rhs: FixedDec) {
        *self = &*self * rhs
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
    let carry = ad > (0xffffffff - bc);
    let msb = ((ad + bc) >> 16) + ((carry as u32) << 16) + b * d;
    (lsb, msb)
}

impl PartialOrd for FixedDec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixedDec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.sign, other.sign) {
            (POS, NEG) => return Ordering::Greater,
            (NEG, POS) => return Ordering::Less,
            _ => (),
        }
        match (self.dec.cmp(&other.dec), self.sign) {
            (Ordering::Less, POS) => return Ordering::Less,
            (Ordering::Less, NEG) => return Ordering::Greater,
            (Ordering::Greater, POS) => return Ordering::Greater,
            (Ordering::Greater, NEG) => return Ordering::Less,
            (Ordering::Equal, _) => (),
        }
        match (self.parts.first().unwrap_or(&0).cmp(other.parts.first().unwrap_or(&0)), self.sign) {
            (Ordering::Less, POS) => Ordering::Less,
            (Ordering::Less, NEG) => Ordering::Greater,
            (Ordering::Greater, POS) => Ordering::Greater,
            (Ordering::Greater, NEG) => Ordering::Less,
            (Ordering::Equal, _) => Ordering::Equal,
        }
    }
}

impl FixedDec {
    pub fn floor(mut self) -> Self {
        if self.sign == NEG {
            let diff = self.parts.len() as i32 - self.dec;
            if diff > 0
                && self.parts[self.dec.max(0) as usize..]
                    .iter()
                    .any(|p| *p != 0)
            {
                self -= Self::one();
            }
        }
        self.parts.truncate(self.dec.max(0) as usize);
        self
    }
    pub fn ceil(mut self) -> Self {
        if self.sign == NEG {
            let diff = self.parts.len() as i32 - self.dec;
            if diff > 0
                && self.parts[self.dec.max(0) as usize..]
                    .iter()
                    .any(|p| *p != 0)
            {
                self -= Self::one();
            }
        }
        self.parts.truncate(self.dec.max(0) as usize);
        self += Self::one();
        self
    }
}
