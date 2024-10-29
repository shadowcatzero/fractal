use super::*;

use std::ops::{Add, AddAssign, Mul, Neg, Shr, Sub, SubAssign};

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

fn add(dest: &mut FixedDec, at: &impl Fn(usize) -> u32, rhs: &FixedDec) {
    let mut carry = false;
    let same_sign = dest.sign == rhs.sign;
    let rhs_offset = rhs.dec - dest.dec;
    for i in (0..dest.parts.len()).rev() {
        let a = at(i);
        let b = rhs.part(i as i32 + rhs_offset);
        (dest.parts[i], carry) = carry_add(a, b, same_sign, carry);
    }
    if same_sign {
        if carry {
            dest.parts.insert(0, 1);
            dest.dec += 1;
        }
    } else if carry {
        dest.sign = !dest.sign
    }
}

fn new_dec(x: &FixedDec, y: &FixedDec) -> (i32, usize) {
    let dec = x.dec.max(y.dec);
    let left_i = -dec;
    let right_i = x.dec_len().max(y.dec_len());
    let len = (right_i - left_i) as usize;
    (dec, len)
}

fn carry_add(a: u32, b: u32, same_sign: bool, carry: bool) -> (u32, bool) {
    if same_sign {
        a.carrying_add(b, carry)
    } else {
        let (res, c) = a.overflowing_sub(b);
        let (res, c2) = res.overflowing_sub(carry as u32);
        (res, c || c2)
    }
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
        self += &(-rhs);
        self
    }
}

impl SubAssign<&FixedDec> for FixedDec {
    fn sub_assign(&mut self, rhs: &FixedDec) {
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
                parts[k] = res;
                let (res, carry2) = parts[k].overflowing_add(carry);
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
