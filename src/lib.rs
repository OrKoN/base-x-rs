/// ! # base_x
/// !
/// ! Encode and decode any base alphabet.

mod alphabet;

pub use alphabet::Alphabet;
use alphabet::INVALID_INDEX;

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
pub fn encode<A: Alphabet>(alphabet: A, input: &[u8]) -> String {
    if input.len() == 0 {
        return String::new();
    }

    let base = alphabet.base() as u16;
    let alphabet = alphabet.as_bytes();

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
pub fn decode<A: Alphabet>(alphabet: A, input: &str) -> Result<Vec<u8>, DecodeError> {
    if input.len() == 0 {
        return Ok(Vec::new());
    }

    let base = alphabet.base() as u16;
    let alphabet_lut = alphabet.lookup_table();

    let mut bytes: Vec<u8> = vec![0];

    for c in input.as_bytes() {
        let mut carry = match alphabet_lut[*c as usize] {
            INVALID_INDEX => return Err(DecodeError),
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

    let leader = alphabet.as_bytes()[0];

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
    extern crate json;
    use self::json::parse;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn works() {
        let mut file = File::open("./fixtures/fixtures.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = parse(&data).unwrap();
        let alphabets = &json["alphabets"];

        for value in json["valid"].members() {
            let alphabet_name = value["alphabet"].as_str().unwrap();
            let input = value["string"].as_str().unwrap();
            let alphabet = alphabets[alphabet_name].as_str().unwrap();

            let decoded = decode(alphabet, input).unwrap();
            let encoded = encode(alphabet, &decoded);
            println!("'{:?}' - '{:?}'", input, encoded);
            assert_eq!(encoded, input);
        }
    }
}
