use encoder::{AsciiEncoder, Utf8Encoder};
use decoder::{AsciiDecoder, Utf8Decoder};
use DecodeError;

use std::ascii::AsciiExt;

const INVALID_INDEX: u8 = 0xFF;

pub trait Alphabet {
    fn encode(&self, input: &[u8]) -> String;

    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError>;
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
        if self.is_ascii() {
            return self.as_bytes().encode(input);
        }

        let alphabet: Vec<char> = self.chars().collect();
        Utf8Encoder::encode(&alphabet, input)
    }

    #[inline(always)]
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError> {
        if self.is_ascii() {
            return self.as_bytes().decode(input);
        }

        let alphabet: Vec<char> = self.chars().collect();
        Utf8Decoder::decode(&alphabet, input)
    }
}
