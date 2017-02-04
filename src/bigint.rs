use std::{mem, ptr, u32};

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

        carry as u32
    }

    /// Check if self is zero.
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.chunks.iter().all(|chunk| *chunk == 0)
    }
}

/// Helper function for transmuting 4 bytes into a chunk.
#[inline(always)]
fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_be(unsafe { mem::transmute(bytes) })
}

/// Helper function for transmuting a slice into a chunk.
#[inline(always)]
fn slice_to_u32(slice: &[u8]) -> u32 {
    debug_assert!(slice.len() == 4);

    unsafe {
        let mut bytes: [u8; 4] = mem::uninitialized();

        ptr::copy_nonoverlapping(
            slice.as_ptr(),
            bytes.as_mut_ptr(),
            4
        );

        bytes_to_u32(bytes)
    }
}

impl<'a> From<&'a [u8]> for BigUint {
    #[inline]
    fn from(mut bytes: &[u8]) -> Self {
        let modulo = bytes.len() % 4;

        let mut chunks = Vec::new();

        if modulo > 0 {
            chunks.reserve(bytes.len() / 4 + 1);

            let mut first = [0u8; 4];

            for (r, w) in bytes[..modulo].iter().zip(first[4-modulo..].iter_mut()) {
                *w = *r;
            }

            bytes = &bytes[modulo..];

            chunks.push(bytes_to_u32(first));
        } else {
            chunks.reserve(bytes.len() / 4);
        }

        for slice in bytes.chunks(4) {
            chunks.push(slice_to_u32(slice))
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
