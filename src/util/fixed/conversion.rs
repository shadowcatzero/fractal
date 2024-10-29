use num_traits::Zero;

use super::{FixedDec, POS};

const INV_SIGN_MASK: u32 = (1 << 31) - 1;
const FRAC_BIT: u32 = 1 << 23;
const FRAC_MASK: u32 = FRAC_BIT - 1;

impl From<f32> for FixedDec {
    fn from(value: f32) -> Self {
        let raw = value.to_bits() & INV_SIGN_MASK;
        let mut exp = (raw >> 23) as i32 - 127;
        let mut frac = raw & FRAC_MASK;
        let mut start = -exp;
        if exp == -127 {
            exp = -126;
            start = -exp;
        } else {
            frac += FRAC_BIT;
            start -= 1;
        }
        let end = -exp + 23;
        let start_i = start.div_euclid(32);
        let end_i = (end - 1).div_euclid(32);
        let mut parts = Vec::new();
        let mut dec = -start_i;
        if start_i == end_i {
            let val = frac << (8 - start.rem_euclid(32));
            if val != 0 {
                parts.push(val);
            }
        } else {
            let s = end.rem_euclid(32);
            let val_high = frac >> s;
            let val_low = frac << (32 - s);
            if val_high != 0 {
                parts.push(val_high);
            } else {
                dec -= 1;
            }
            if val_low != 0 {
                parts.push(val_low);
            }
        }
        if parts.is_empty() {
            dec = 0;
        }
        let s = Self {
            sign: POS,
            dec,
            parts,
        };
        if value.is_sign_negative() {
            -&s
        } else {
            s
        }
    }
}

impl From<FixedDec> for f32 {
    fn from(value: FixedDec) -> Self {
        Self::from(&value)
    }
}

impl From<&FixedDec> for f32 {
    fn from(value: &FixedDec) -> Self {
        if value.is_zero() {
            return if value.sign == POS { 0.0 } else { -0.0 };
        }
        let mut sign = 0;
        let value = if value.is_neg() {
            sign = 1 << 31;
            &-value
        } else {
            value
        };
        let mut skip_count = 0;
        let mut iter = value.parts.iter().peekable();

        while let Some(0) = iter.peek() {
            skip_count += 1;
            iter.next();
        }

        let Some(v) = iter.next() else {
            return 0.0;
        };
        let mut start = v.leading_zeros() + 1;
        let exp_i = (value.dec - skip_count) * 32 - start as i32;
        let mut frac_sh = 0;
        let exp = if exp_i >= -127 {
            if exp_i == -127 {
                start -= 1;
            }
            (exp_i + 127) as u32
        } else {
            let sh = -(exp_i + 32 * 4 - 1);
            if sh < 23 {
                start -= 1;
                frac_sh = sh;
                0
            } else {
                return 0.0;
            }
        };
        let frac = if start > 9 {
            let sh = start - 9;
            (v << sh) + iter.next().copied().map(|v| v >> (32 - sh)).unwrap_or(0)
        } else {
            v >> (9 - start)
        } & !(1 << 23);
        let res = (frac >> frac_sh) + (exp << 23) + sign;
        f32::from_bits(res)
    }
}
