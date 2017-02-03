use std::collections::HashMap;

use DecodeError;

pub struct AsciiDecoder;
pub struct Utf8Decoder;

macro_rules! decode {
    ($alpha:ident, $input:ident, $iter:ident, $c:pat => $carry:expr) => ({
        if $input.len() == 0 {
            return Ok(Vec::new());
        }

        let base = $alpha.len() as u16;

        let mut bytes = Vec::with_capacity($input.len());
        bytes.push(0u8);

        for $c in $input.$iter() {
            let mut carry = $carry as u16;

            for byte in bytes.iter_mut() {
                carry += base * *byte as u16;
                *byte = carry as u8;
                carry >>= 8;
            }

            while carry > 0 {
                bytes.push(carry as u8);
                carry >>= 8;
            }
        }

        let leader = $alpha[0];

        let leaders = $input
            .$iter()
            .take($input.len() - 1)
            .take_while(|byte| *byte == leader)
            .map(|_| 0);

        bytes.extend(leaders);
        bytes.reverse();
        Ok(bytes)
    })
}

impl AsciiDecoder {
    #[inline(always)]
    pub fn decode(alphabet: &[u8], lookup: [u8; 256], input: &str) -> Result<Vec<u8>, DecodeError> {
        decode!(alphabet, input, bytes, c => match lookup[c as usize] {
            0xFF => return Err(DecodeError),
            index => index
        })
    }
}

impl Utf8Decoder {
    #[inline(always)]
    pub fn decode(alphabet: &[char], lookup: HashMap<char, usize>, input: &str) -> Result<Vec<u8>, DecodeError> {
        decode!(alphabet, input, chars, c => *lookup.get(&c).ok_or(DecodeError)?)
    }
}
