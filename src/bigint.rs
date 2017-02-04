use std::{ptr, u32};

/// This is a pretty naive implementation of a BigUint abstracting all
/// math out to a vector of `u32` chunks.
///
/// It can only do 3 things:
/// - Be instantiated from an arbitrary byte slice
/// - Do a division by `u32`, mutating self and returning the remainder.
/// - Check if it's zero.
///
/// Turns out those are all the operations you need to encode base58,
/// or anything else, really.
pub struct BigUint {
    pub chunks: Vec<u32>
}

impl BigUint {
    /// Divide self by `divider`, return the remainder of the operation.
    #[inline(always)]
    pub fn rem_div(&mut self, divider: u32) -> u32 {
        let mut carry = 0u64;

        for chunk in self.chunks.iter_mut() {
            carry = (carry << 32) | *chunk as u64;
            *chunk = (carry / divider as u64) as u32;
            carry %= divider as u64;
        }

        if self.chunks[0] == 0 {
            self.chunks.remove(0);
        }

        carry as u32
    }

    /// Check if self is zero.
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.chunks.iter().all(|chunk| *chunk == 0)
    }
}

impl<'a> From<&'a [u8]> for BigUint {
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        let modulo = bytes.len() % 4;

        let len = bytes.len() / 4 + (modulo > 0) as usize;

        let mut chunks = Vec::with_capacity(len);

        unsafe {
            chunks.set_len(len);
            *chunks.get_unchecked_mut(0) = 0u32;

            ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                (chunks.as_mut_ptr() as *mut u8).offset(modulo as isize),
                bytes.len()
            );
        }

        for chunk in chunks.iter_mut() {
            *chunk = u32::from_be(*chunk);
        }

        BigUint {
            chunks: chunks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigUint;

    #[test]
    fn big_uint_from_bytes() {
        let bytes: &[u8] = &[
                      0xDE,0xAD,0x00,0x00,0x00,0x13,
            0x37,0xAD,0x00,0x00,0x00,0x00,0xDE,0xAD,
        ];

        let big = BigUint::from(bytes);

        assert_eq!(big.chunks, vec![0x0000DEAD, 0x00000013, 0x37AD0000, 0x0000DEAD]);
    }

    #[test]
    fn big_uint_rem_div() {
        let mut big = BigUint {
            chunks: vec![0x136AD712,0x84322759]
        };

        let rem = big.rem_div(58);
        let merged = ((big.chunks[0] as u64) << 32) | big.chunks[1] as u64;

        assert_eq!(merged, 0x136AD71284322759 / 58);
        assert_eq!(rem as u64, 0x136AD71284322759 % 58);
    }
}
