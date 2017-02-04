use bigint::BigUint;

pub struct AsciiEncoder;
pub struct Utf8Encoder;

macro_rules! encode {
    ($alpha:ident, $input:ident) => ({
        if $input.len() == 0 {
            return String::new();
        }

        let base = $alpha.len() as u32;

        let mut big = BigUint::from($input);
        let mut out = Vec::with_capacity($input.len());

        while !big.is_zero() {
            out.push($alpha[big.rem(base) as usize]);
            big /= base;
        }

        let leaders = $input
            .iter()
            .take($input.len() - 1)
            .take_while(|i| **i == 0)
            .map(|_| $alpha[0]);

        out.extend(leaders);

        out
    })
}

impl AsciiEncoder {
    #[inline(always)]
    pub fn encode(alphabet: &[u8], input: &[u8]) -> String {
        let mut out = encode!(alphabet, input);

        out.reverse();

        String::from_utf8(out).expect("Alphabet must be ASCII")
    }
}

impl Utf8Encoder {
    #[inline(always)]
    pub fn encode(alphabet: &[char], input: &[u8]) -> String {
        let out = encode!(alphabet, input);

        out.iter().rev().map(|char| *char).collect()
    }
}
