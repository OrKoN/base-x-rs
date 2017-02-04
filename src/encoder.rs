use bigint::BigUint;

pub struct AsciiEncoder;
pub struct Utf8Encoder;

macro_rules! encode {
    ($alpha:ident, $input:ident) => ({
        if $input.len() == 0 {
            return String::new();
        }

        let base = $alpha.len() as u32;

        // Convert the input byte array to a BigUint
        let mut big = BigUint::from_bytes_be($input);
        let mut out = Vec::with_capacity($input.len());

        // Find the highest power of `base` that fits in `u32`
        let big_pow = 32 / (32 - base.leading_zeros());
        let big_base = base.pow(big_pow);

        'fast: loop {
            // Instead of diving by `base`, we divide by the `big_base`,
            // giving us a bigger remainder that we can further subdivide
            // by the original `base`. This greatly (in case of base58 it's
            // a factor of 5) reduces the amount of divisions that need to
            // be done on BigUint, delegating the hard work to regular `u32`
            // operations, which are blazing fast.
            let mut big_rem = big.rem_div(big_base);

            if big.is_zero() {
                loop {
                    out.push($alpha[(big_rem % base) as usize]);
                    big_rem /= base;

                    if big_rem == 0 {
                        break 'fast; // teehee
                    }
                }
            } else {
                for _ in 0..big_pow {
                    out.push($alpha[(big_rem % base) as usize]);
                    big_rem /= base;
                }
            }
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
