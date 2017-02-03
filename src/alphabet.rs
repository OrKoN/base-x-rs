use std::collections::HashMap;

use encoder::{AsciiEncoder, Utf8Encoder};
use decoder::{AsciiDecoder, Utf8Decoder};

use DecodeError;

const INVALID_INDEX: u8 = 0xFF;

pub trait Alphabet {
    fn encode(&self, input: &[u8]) -> String;

    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError>;
}

#[derive(Clone, Copy)]
pub struct Binary;

impl Alphabet for Binary {
    #[inline(always)]
    fn encode(&self, input: &[u8]) -> String {
        let cap = input.len() * 8;
        let mut out = Vec::with_capacity(cap);

        unsafe {
            out.set_len(cap);

            let ptr = out.as_mut_ptr();
            let mut i = 0isize;

            for byte in input {
                *ptr.offset(i)     = (*byte >> 7) + 0x30;
                *ptr.offset(i + 1) = ((*byte >> 6) & 1) + 0x30;
                *ptr.offset(i + 2) = ((*byte >> 5) & 1) + 0x30;
                *ptr.offset(i + 3) = ((*byte >> 4) & 1) + 0x30;
                *ptr.offset(i + 4) = ((*byte >> 3) & 1) + 0x30;
                *ptr.offset(i + 5) = ((*byte >> 2) & 1) + 0x30;
                *ptr.offset(i + 6) = ((*byte >> 1) & 1) + 0x30;
                *ptr.offset(i + 7) = (*byte & 1) + 0x30;
                i += 8;
            }

            String::from_utf8_unchecked(out)
        }
    }

    #[inline(always)]
    fn decode(&self, _input: &str) -> Result<Vec<u8>, DecodeError> {
        Ok(Vec::new())
    }
}

impl<'a> Alphabet for &'a [u8] {
    #[inline(always)]
    fn encode(&self, input: &[u8]) -> String {
        AsciiEncoder::encode(*self, input)
    }

    #[inline(always)]
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError> {
        let mut lookup = [INVALID_INDEX; 256];

        for (i, byte) in self.iter().enumerate() {
            lookup[*byte as usize] = i as u8;
        }

        AsciiDecoder::decode(*self, lookup, input)
    }
}

impl<'a> Alphabet for &'a str {
    #[inline(always)]
    fn encode(&self, input: &[u8]) -> String {
        let alphabet: Vec<char> = self.chars().collect();
        Utf8Encoder::encode(&alphabet, input)
    }

    #[inline(always)]
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError> {
        let alphabet: Vec<char> = self.chars().collect();

        let mut map = HashMap::with_capacity(alphabet.len());

        for (index, ch) in self.chars().enumerate() {
            map.insert(ch, index);
        }

        Utf8Decoder::decode(&alphabet, map, input)
    }
}

// #[cfg(test)]
// mod test {
//     use super::{Alphabet, CharLookup};
//     use std::collections::HashMap;

//     #[test]
//     fn lookup_str() {
//         let alphabet = "abcd";

//         let lookup: HashMap<char, usize> = alphabet.lookup_table();

//         assert_eq!(CharLookup::get(&lookup, 'a'), Some(0));
//         assert_eq!(CharLookup::get(&lookup, 'b'), Some(1));
//         assert_eq!(CharLookup::get(&lookup, 'c'), Some(2));
//         assert_eq!(CharLookup::get(&lookup, 'd'), Some(3));
//         assert_eq!(CharLookup::get(&lookup, 'e'), None);
//         assert_eq!(CharLookup::get(&lookup, '7'), None);
//         assert_eq!(CharLookup::get(&lookup, '$'), None);
//     }

//     #[test]
//     fn lookup_bytes() {
//         let alphabet: &[u8] = b"qwer";

//         let lookup: [u8; 256] = alphabet.lookup_table();

//         assert_eq!(lookup.get('q'), Some(0));
//         assert_eq!(lookup.get('w'), Some(1));
//         assert_eq!(lookup.get('e'), Some(2));
//         assert_eq!(lookup.get('r'), Some(3));
//         assert_eq!(lookup.get('t'), None);
//         assert_eq!(lookup.get('*'), None);
//         assert_eq!(lookup.get('_'), None);
//     }
// }
