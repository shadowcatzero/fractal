mod conversion;
mod op;
#[cfg(test)]
mod test;

use num_traits::Zero;
use std::fmt::{Binary, Display};

const POS: bool = false;
const NEG: bool = true;

// dec is from the left instead of from the right
// because this is a fractal viewer, so it's expected
// that people zoom in instead of out; parts also has
// the most significant u32 at 0, so to zoom in you just
// have to add to the vec instead of insert at 0
// Might want to try to make it abstract over the direction
// of growth, or use a VecDeque, but doesn't seem worth for
// now
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedDec {
    sign: bool,
    dec: i32,
    parts: Vec<u32>,
}

impl FixedDec {
    pub fn one() -> Self {
        Self {
            sign: POS,
            dec: 1,
            parts: vec![1],
        }
    }

    pub fn from_parts(sign: bool, dec: i32, parts: Vec<u32>) -> Self {
        Self { sign, dec, parts }
    }

    pub fn zeros() -> Self {
        Self::zero()
    }

    pub fn dec_len(&self) -> i32 {
        self.parts.len() as i32 - self.dec
    }

    pub fn part(&self, i: i32) -> u32 {
        let Ok(i): Result<usize, _> = i.try_into() else {
            return 0;
        };
        self.parts.get(i).cloned().unwrap_or(0)
    }

    pub fn is_pos(&self) -> bool {
        !self.sign
    }

    pub fn is_neg(&self) -> bool {
        self.sign
    }

    pub fn trim(&mut self) {
        let rem_beg = self.parts.iter().take_while(|&&x| x == 0).count();
        self.parts.drain(0..rem_beg);
        let rem_end = self.parts.iter().rev().take_while(|&&x| x == 0).count();
        self.parts.truncate(self.parts.len() - rem_end);
        if self.parts.is_empty() {
            self.dec = 0;
        } else {
            self.dec -= rem_beg as i32;
        }
        if self.parts.is_empty() {
            self.sign = POS;
        }
    }

    pub fn set_whole_len(&mut self, len: i32) {
        let diff = len - self.dec;
        let remove = 0..usize::try_from(-diff).unwrap_or(0).min(self.parts.len());
        self.parts.splice(remove, (0..diff).map(|_| 0));
        self.dec += diff;
        if self.parts.is_empty() {
            self.dec = 0;
        }
    }

    pub fn with_whole_len(mut self, len: i32) -> Self {
        self.set_whole_len(len);
        self
    }

    pub fn set_dec_len(&mut self, len: i32) {
        let len = usize::try_from(len + self.dec).unwrap_or(0);
        self.parts.resize(len, 0);
        if self.parts.is_empty() {
            self.dec = 0;
        }
    }

    pub fn with_dec_len(mut self, len: i32) -> Self {
        self.set_dec_len(len);
        self
    }

    pub fn with_lens(self, whole_len: i32, dec_len: i32) -> Self {
        self.with_whole_len(whole_len).with_dec_len(dec_len)
    }

    pub fn split_whole_dec(&self) -> (FixedDec, FixedDec) {
        let take_skip = usize::try_from(self.dec).unwrap_or(0);
        let whole = FixedDec {
            sign: self.sign,
            dec: self.dec,
            parts: self.parts.iter().take(take_skip).copied().collect(),
        };
        let dec = if self.dec >= 0 { 0 } else { self.dec };
        let dec = FixedDec {
            sign: self.sign,
            dec,
            parts: self.parts.iter().skip(take_skip).copied().collect(),
        };
        (whole, dec)
    }

    pub fn set_precision(&mut self, prec: usize) {
        self.parts.resize(prec, 0);
        if self.parts.is_empty() {
            self.dec = 0;
        }
    }

    pub fn to_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend((self.sign as u32).to_le_bytes());
        bytes.extend(self.dec.to_le_bytes());
        bytes.extend(self.parts.iter().flat_map(|p| p.to_le_bytes()));
    }

    pub fn parts(&self) -> &[u32] {
        &self.parts
    }

    pub fn negate(&mut self) {
        if !self.is_zero() {
            self.sign = !self.sign;
        }
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
            write!(f, "00000000000000000000000000000000")?;
            for _ in 0..(-self.dec - 1) {
                write!(f, "_00000000000000000000000000000000")?;
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
        let diff = usize::try_from(self.dec)
            .unwrap_or(0)
            .saturating_sub(self.parts.len());
        for _ in 0..diff {
            write!(f, "_00000000000000000000000000000000")?;
        }
        Ok(())
    }
}
