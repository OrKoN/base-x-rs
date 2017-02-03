pub struct AsciiEncoder;
pub struct Utf8Encoder;

macro_rules! encode {
    ($alpha:ident, $input:ident, $dig:ty) => ({
        if $input.len() == 0 {
            return String::new();
        }

        let base = $alpha.len() as u16;

        let mut digits: Vec<$dig> = Vec::with_capacity($input.len());
        digits.push(0);

        for c in $input {
            let mut carry = *c as u16;

            for digit in digits.iter_mut() {
                carry |= (*digit as u16) << 8;
                *digit = (carry % base) as $dig;
                carry /= base;
            }

            while carry > 0 {
                digits.push((carry % base) as $dig);
                carry /= base;
            }
        }

        let leaders = $input
            .iter()
            .take($input.len() - 1)
            .take_while(|i| **i == 0)
            .map(|_| 0);

        digits.extend(leaders);

        digits
    })
}

impl AsciiEncoder {
    #[inline(always)]
    pub fn encode(alphabet: &[u8], input: &[u8]) -> String {
        let mut digits = encode!(alphabet, input, u8);

        digits.reverse();

        for digit in digits.iter_mut() {
            *digit = alphabet[*digit as usize];
        }

        String::from_utf8(digits).expect("Alphabet must be ASCII")
    }
}

impl Utf8Encoder {
    #[inline(always)]
    pub fn encode(alphabet: &[char], input: &[u8]) -> String {
        let digits = encode!(alphabet, input, u8);

        let encoded = digits
            .iter()
            .rev()
            .map(|digit| alphabet[*digit as usize]);

        let mut result = String::new();
        result.extend(encoded);

        result
    }
}
