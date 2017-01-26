/// ! # base_x
/// !
/// ! Encode and decode any base alphabet.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EncodeError;

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to encode the given data")
    }
}

impl Error for EncodeError {
    fn description(&self) -> &str {
        "Can not encode the provided data"
    }
}

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
pub fn encode(alphabet: &str, input: Vec<u16>) -> Result<String, EncodeError> {
    if input.len() == 0 {
        return Ok(String::new());
    }

    let base = alphabet.len() as u16;
    let mut alphabet_map = HashMap::with_capacity(alphabet.len());

    for (i, c) in alphabet.chars().enumerate() {
        alphabet_map.insert(i, c);
    }

    let mut digits: Vec<u16> = Vec::with_capacity(input.len());

    digits.push(0);

    for c in &input {
        let mut j = 0;
        let mut carry = *c;
        while j < digits.len() {
            carry = carry + (digits[j] << 8);
            digits[j] = carry % base;
            carry = (carry / base) | 0;
            j += 1;
        }

        while carry > 0 {
            digits.push(carry % base);
            carry = (carry / base) | 0;
        }
    }

    let leaders = input
        .iter()
        .take(input.len() - 1)
        .take_while(|i| **i == 0)
        .map(|_| 0);

    digits.extend(leaders);

    let mut output = String::new();

    for i in (0..digits.len()).rev() {
        let char = try!(alphabet_map.get(&(digits[i] as usize)).ok_or(EncodeError));
        output.push(*char)
    }

    Ok(output)
}

/// Deocde an input vector using the given alphabet.
pub fn decode(alphabet: &str, input: &str) -> Result<Vec<u16>, DecodeError> {
    if input.len() == 0 {
        return Ok(vec![]);
    }

    let base = alphabet.len() as u16;
    let leader = try!(alphabet.chars().nth(0).ok_or(DecodeError));
    let mut alphabet_map = HashMap::new();

    for (i, c) in alphabet.chars().enumerate() {
        alphabet_map.insert(c, i as u16);
    }

    let mut bytes = vec![0];
    for c in input.chars() {
        let carry = try!(alphabet_map.get(&c).ok_or(DecodeError));
        let mut carry = carry.clone();
        for b in &mut bytes {
            carry = carry + *b * base;
            *b = carry & 0xff;
            carry = carry >> 8;
        }

        while carry > 0 {
            bytes.push(carry & 0xff);
            carry = carry >> 8;
        }
    }

    let leaders = input.chars()
        .into_iter()
        .take(input.len() - 1)
        .take_while(|char| *char == leader)
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
            let decoded = decode(&alphabet, &input).unwrap();
            let encoded = encode(&alphabet, decoded).unwrap();
            println!("'{:?}' - '{:?}'", input, encoded);
            assert_eq!(encoded, input);
        }
    }
}
