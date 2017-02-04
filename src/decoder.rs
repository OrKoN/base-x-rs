use DecodeError;
use bigint::BigUint;

pub struct AsciiDecoder;
pub struct Utf8Decoder;

macro_rules! decode {
    ($alpha:ident, $input:ident, $iter:ident, $c:pat => $carry:expr) => ({
        if $input.len() == 0 {
            return Ok(Vec::new());
        }

        let base = $alpha.len() as u32;

        let mut big = BigUint::with_capacity(4);

        for $c in $input.$iter() {
            big.add_mul($carry as u32, base);
        }

        let mut bytes = big.into_bytes_be();

        let leader = $alpha[0];

        let leaders = $input
            .$iter()
            .take_while(|byte| *byte == leader)
            .count();

        for _ in 0..leaders {
            bytes.insert(0, 0);
        }

        Ok(bytes)
    })
}

impl AsciiDecoder {
    #[inline(always)]
    pub fn decode(alphabet: &[u8], lookup: [u8; 256], input: &str) -> Result<Vec<u8>, DecodeError> {
        decode!(
            alphabet,
            input,
            bytes,
            c => match lookup[c as usize] {
                0xFF => return Err(DecodeError),
                index => index
            }
        )
    }
}

impl Utf8Decoder {
    #[inline(always)]
    pub fn decode(alphabet: &[char], input: &str) -> Result<Vec<u8>, DecodeError> {
        decode!(
            alphabet,
            input,
            chars,
            // Vector find is faster than HashMap even for Base58
            c => alphabet
                .iter()
                .enumerate()
                .find(|&(_, ch)| *ch == c)
                .map(|(i, _)| i)
                .ok_or(DecodeError)?
        )
    }
}
