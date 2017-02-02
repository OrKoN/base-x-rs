//! # base_x
//!
//! Encode and decode any base alphabet.
//!
//! ## Installation
//!
//! Add this to `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! base-x = "0.2.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! extern crate base_x;
//!
//! fn main() {
//!   let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
//!   let encoded = base_x::encode("01", &decoded);
//!  assert_eq!(encoded, "11111111000000001111111100000000");
//! }
//! ```

mod alphabet;

pub use alphabet::{Alphabet, CharLookup};

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
        .map(|digit| alphabet.get(*digit as usize));

    let mut result = String::new();
    result.extend(encoded);

    result
}

/// Decode an input vector using the given alphabet.
pub fn decode<A: Alphabet>(alphabet: A, input: &str) -> Result<Vec<u8>, DecodeError> {
    if input.len() == 0 {
        return Ok(Vec::new());
    }

    let base = alphabet.base() as u16;
    let lookup = alphabet.lookup_table();

    let mut bytes: Vec<u8> = vec![0];

    for c in input.chars() {
        let mut carry = lookup.get(c).ok_or(DecodeError)? as u16;

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

    let leader = alphabet.get(0) as u8;

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

            // Alphabet works as unicode
            let decoded = decode(alphabet, input).unwrap();
            let encoded = encode(alphabet, &decoded);
            assert_eq!(encoded, input);

            // Alphabet works as ASCII
            let decoded = decode(alphabet.as_bytes(), input).unwrap();
            let encoded = encode(alphabet.as_bytes(), &decoded);
            assert_eq!(encoded, input);
        }
    }

    #[test]
    fn is_unicode_sound() {
        // binary, kinda...
        let alphabet = "😐😀";

        let encoded = encode(alphabet, &[0xff,0x00,0xff,0x00]);
        let decoded = decode(alphabet, &encoded).unwrap();

        assert_eq!(encoded, "😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐");
        assert_eq!(decoded, &[0xff,0x00,0xff,0x00]);
    }
}
