//! The small subset of arbitrary-precision arithmetic needed by the counter.
//! Limbs are little-endian base 2^32; zero has no limbs.

use std::fmt;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Natural(Vec<u32>);

impl Natural {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn one() -> Self {
        Self(vec![1])
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    pub fn bit_len(&self) -> usize {
        self.0.last().map_or(0, |last| {
            (self.0.len() - 1) * 32 + (32 - last.leading_zeros() as usize)
        })
    }

    pub fn add_assign(&mut self, other: &Self) {
        self.0.resize(self.0.len().max(other.0.len()), 0);
        let mut carry = 0u64;
        for (i, limb) in self.0.iter_mut().enumerate() {
            let sum = u64::from(*limb) + u64::from(*other.0.get(i).unwrap_or(&0)) + carry;
            *limb = sum as u32;
            carry = sum >> 32;
        }
        if carry != 0 {
            self.0.push(carry as u32);
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut result = vec![0u32; self.0.len() + other.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in other.0.iter().enumerate() {
                let value = u64::from(a) * u64::from(b) + u64::from(result[i + j]) + carry;
                result[i + j] = value as u32;
                carry = value >> 32;
            }
            result[i + other.0.len()] = carry as u32;
        }
        while result.last() == Some(&0) {
            result.pop();
        }
        Self(result)
    }

    pub fn shl_assign(&mut self, bits: usize) {
        if self.is_zero() || bits == 0 {
            return;
        }
        let words = bits / 32;
        let shift = bits % 32;
        if shift != 0 {
            let mut carry = 0u64;
            for limb in &mut self.0 {
                let value = (u64::from(*limb) << shift) | carry;
                *limb = value as u32;
                carry = value >> 32;
            }
            if carry != 0 {
                self.0.push(carry as u32);
            }
        }
        if words != 0 {
            let old_len = self.0.len();
            self.0.resize(old_len + words, 0);
            self.0.copy_within(0..old_len, words);
            self.0[..words].fill(0);
        }
    }

    fn div_small(&mut self, divisor: u32) -> u32 {
        let mut remainder = 0u64;
        for limb in self.0.iter_mut().rev() {
            let value = (remainder << 32) | u64::from(*limb);
            *limb = (value / u64::from(divisor)) as u32;
            remainder = value % u64::from(divisor);
        }
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
        remainder as u32
    }
}

impl From<u128> for Natural {
    fn from(mut value: u128) -> Self {
        let mut limbs = Vec::new();
        while value != 0 {
            limbs.push(value as u32);
            value >>= 32;
        }
        Self(limbs)
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let mut value = self.clone();
        let mut chunks = Vec::new();
        while !value.is_zero() {
            chunks.push(value.div_small(1_000_000_000));
        }
        write!(f, "{}", chunks.pop().unwrap())?;
        for chunk in chunks.into_iter().rev() {
            write!(f, "{chunk:09}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_matches_u128_across_limb_boundaries() {
        let values = [0, 1, 2, (1u128 << 32) - 1, 1 << 32, (1 << 64) - 1];
        for a in values {
            for b in values {
                let mut sum = Natural::from(a);
                sum.add_assign(&Natural::from(b));
                assert_eq!(sum.to_string(), (a + b).to_string());
                assert_eq!(
                    Natural::from(a).mul(&Natural::from(b)).to_string(),
                    (a * b).to_string()
                );
            }
            for shift in [0, 1, 31, 32, 33, 63, 64] {
                let mut shifted = Natural::from(a);
                shifted.shl_assign(shift);
                assert_eq!(shifted.to_string(), (a << shift).to_string());
            }
        }
    }

    #[test]
    fn exact_far_beyond_machine_words() {
        let mut power = Natural::one();
        power.shl_assign(200);
        assert_eq!(power.bit_len(), 201);
        assert_eq!(
            power.to_string(),
            "1606938044258990275541962092341162602522202993782792835301376"
        );
        let mut half = Natural::one();
        half.shl_assign(100);
        assert_eq!(half.mul(&half), power);
        let all_ones = Natural::from(u128::MAX);
        let square = all_ones.mul(&all_ones);
        let mut twice = all_ones.clone();
        twice.shl_assign(1);
        let mut recovered = square;
        recovered.add_assign(&twice);
        recovered.add_assign(&Natural::one());
        let mut expected = Natural::one();
        expected.shl_assign(256);
        assert_eq!(recovered, expected);
    }
}
