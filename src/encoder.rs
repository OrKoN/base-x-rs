pub struct AsciiEncoder;
pub struct Utf8Encoder;

impl AsciiEncoder {
    #[inline(always)]
    pub fn encode(alphabet: &[u8], input: &[u8]) -> String {
        if input.len() == 0 {
            return String::new();
        }

        let base = alphabet.len() as u16;

        let mut digits = Vec::with_capacity(input.len());
        digits.push(0u8);

        for c in input {
            let mut carry = *c as u16;

            for digit in digits.iter_mut() {
                carry |= (*digit as u16) << 8;
                *digit = (carry % base) as u8;
                carry /= base;
            }

            while carry > 0 {
                digits.push((carry % base) as u8);
                carry /= base;
            }
        }

        let leaders = input
            .iter()
            .take(input.len() - 1)
            .take_while(|i| **i == 0)
            .map(|_| 0);

        digits.extend(leaders);
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
        if input.len() == 0 {
            return String::new();
        }

        let base = alphabet.len() as u16;

        let mut digits: Vec<u16> = Vec::with_capacity(input.len());

        digits.push(0);

        for c in input {
            let mut carry = *c as u16;

            for digit in digits.iter_mut() {
                carry += *digit << 8;
                *digit = carry % base;
                carry /= base;
            }

            while carry > 0 {
                digits.push(carry % base);
                carry /= base;
            }
        }

        let leaders = input
            .iter()
            .take(input.len() - 1)
            .take_while(|i| **i == 0)
            .map(|_| 0);

        digits.extend(leaders);

        let encoded = digits
            .iter()
            .rev()
            .map(|digit| alphabet[*digit as usize]);

        let mut result = String::new();
        result.extend(encoded);

        result
    }
}
