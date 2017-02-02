/// ! # base_x
/// !
/// ! Encode and decode any base alphabet.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode the given data")
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        "Can not decode the provided data"
    }
}

/// Encode an input vector using the given alphabet.
pub fn encode(alphabet: &[u8], input: &[u8]) -> String {
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
        .map(|digit| alphabet[*digit as usize])
        .collect();

    String::from_utf8(encoded).expect("Result has to be ASCII")
}

/// Decode an input vector using the given alphabet.
pub fn decode(alphabet: &[u8], input: &str) -> Result<Vec<u8>, DecodeError> {
    if input.len() == 0 {
        return Ok(Vec::new());
    }

    let base = alphabet.len() as u16;

    // Alphabet cannot be longer than 255 bytes, so 0xFF is a safe bet for an
    // invalid index.
    const INVALID: u8 = 0xFF;

    // Ideally this lookup table would be generated on compile time for
    // All the alphabets. That said, this should be pretty darn fast anyway.
    let mut alphabet_lut = [INVALID; 256];

    for (i, byte) in alphabet.iter().enumerate() {
        alphabet_lut[*byte as usize] = i as u8;
    }

    let mut bytes: Vec<u8> = vec![0];

    for c in input.as_bytes() {
        let mut carry = match alphabet_lut[*c as usize] {
            INVALID => return Err(DecodeError),
            carry => carry,
        } as u16;

        for byte in bytes.iter_mut() {
            carry += (*byte as u16) * base;
            *byte = carry as u8;
            carry >>= 8;
        }

        while carry > 0 {
            bytes.push(carry as u8);
            carry >>= 8;
        }
    }

    let leader = alphabet[0];

    let leaders = input
        .bytes()
        .take(input.len() - 1)
        .take_while(|byte| *byte == leader)
        .map(|_| 0);

    bytes.extend(leaders);
    bytes.reverse();
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::encode;
    use super::decode;
    extern crate rustc_serialize;
    use self::rustc_serialize::json::{self, Json};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn works() {
        let mut file = File::open("./fixtures/fixtures.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = Json::from_str(&data).unwrap();
        let alphabets = json.as_object().unwrap().get("alphabets").unwrap().as_object().unwrap();
        let valid = json.as_object().unwrap().get("valid").unwrap().as_array().unwrap();

        for value in valid {
            let obj = value.as_object().unwrap();
            let alphabet_name = obj.get("alphabet").unwrap().to_string();
            let alphabet_name: String = json::decode(&alphabet_name).unwrap();
            let input = obj.get("string").unwrap().to_string();
            let input: String = json::decode(&input).unwrap();
            let alphabet = alphabets.get(&alphabet_name).unwrap().to_string();
            let alphabet: String = json::decode(&alphabet).unwrap();
            let decoded = decode(alphabet.as_bytes(), &input).unwrap();
            let encoded = encode(alphabet.as_bytes(), &decoded);
            println!("'{:?}' - '{:?}'", input, encoded);
            assert_eq!(encoded, input);
        }
    }
}
