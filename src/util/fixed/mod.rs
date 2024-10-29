mod conversion;
mod op;
#[cfg(test)]
mod test;

use num_traits::Zero;
use std::fmt::{Binary, Display};

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
        let rem_beg = self
            .parts
            .iter()
            .take_while(|&&x| x == 0)
            .count();
        self.parts.drain(0..rem_beg);
        let rem_end = self.parts.iter().rev().take_while(|&&x| x == 0).count();
        self.parts.truncate(self.parts.len() - rem_end);
        if self.parts.is_empty() {
            self.dec = 0;
        } else {
            self.dec -= rem_beg as i32;
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
