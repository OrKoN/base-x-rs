use std::{ops, mem, ptr, u32};

const MASK_BASE: u32 = 0xFFFFFFFF;

pub struct BigUint {
    pub chunks: Vec<u32>
}

impl ops::ShrAssign<u8> for BigUint {
    #[inline(always)]
    fn shr_assign(&mut self, shift: u8) {
        let mut carry = 0u32;
        let unshift = 32 - shift;
        let mask = MASK_BASE >> unshift;

        for chunk in self.chunks.iter_mut() {
            let new_carry = (*chunk & mask) << unshift;
            *chunk >>= shift;
            *chunk |= carry;
            carry = new_carry;
        }
    }
}

impl ops::DivAssign<u32> for BigUint {
    #[inline(always)]
    fn div_assign(&mut self, divider: u32) {
        let mut carry = 0u64;

        for chunk in self.chunks.iter_mut() {
            carry |= *chunk as u64;
            *chunk = (carry / divider as u64) as u32;
            carry = (carry % divider as u64) << 32;
        }
    }
}

impl BigUint {
    #[inline(always)]
    pub fn rem(&self, divider: u32) -> u32 {
        self.chunks.last().unwrap() % divider
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.chunks.iter().all(|chunk| *chunk == 0)
    }
}

#[inline(always)]
fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_be(unsafe { mem::transmute(bytes) })
}

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
    fn big_uint_shr() {
        let mut big = BigUint {
            chunks: vec![0x0000DEAD,0x00000013, 0x37AD0000, 0x0000DEAD]
        };

        big >>= 8;

        assert_eq!(big.chunks, vec![0x000000DE, 0xAD000000, 0x1337AD00, 0x000000DE]);
    }

    #[test]
    fn big_uint_rem() {
        let big = BigUint {
            chunks: vec![1337]
        };

        assert_eq!(big.rem(100), 37);
    }

    #[test]
    fn big_uint_div() {
        let mut big = BigUint {
            chunks: vec![0x136AD712,0x84322759]
        };

        big /= 58;

        let merged = ((big.chunks[0] as u64) << 32) | big.chunks[1] as u64;

        assert_eq!(merged, 0x136AD71284322759 / 58);
    }
}
