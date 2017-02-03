#[macro_use]
extern crate bencher;
extern crate rand;
extern crate base_x;

use bencher::Bencher;
use base_x::{encode, decode, Alphabet};


fn random_input(size: usize) -> Vec<u8> {
    let mut v = vec![0; size];

    for x in v.iter_mut() {
        *x = rand::random()
    }

    v
}

fn test_decode<A: Alphabet + Copy>(bench: &mut Bencher, alph: A) {
    let input = random_input(100);
    let out = encode(alph, &input);

    bench.iter(|| {
        decode(alph, &out).unwrap()
    });
}

fn test_encode<A: Alphabet + Copy>(bench: &mut Bencher, alph: A) {
    let input = random_input(100);

    bench.iter(|| {
        encode(alph, &input)
    });
}

// Actual benchmarks

// Encode UTF-8
fn encode_base2_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "01";
    test_encode(bench, ALPH);
}

fn encode_base16_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "0123456789abcdef";
    test_encode(bench, ALPH);
}

fn encode_base58_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    test_encode(bench, ALPH);
}

// Encode ASCII
fn encode_base2_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"01";
    test_encode(bench, ALPH);
}

fn encode_base16_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"0123456789abcdef";
    test_encode(bench, ALPH);
}

fn encode_base58_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    test_encode(bench, ALPH);
}

// Decode UTF-8
fn decode_base2_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "01";
    test_decode(bench, ALPH);
}

fn decode_base16_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "0123456789abcdef";
    test_decode(bench, ALPH);
}

fn decode_base58_utf8(bench: &mut Bencher) {
    const ALPH: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    test_decode(bench, ALPH);
}

// Decode ASCII
fn decode_base2_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"01";
    test_decode(bench, ALPH);
}

fn decode_base16_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"0123456789abcdef";
    test_decode(bench, ALPH);
}

fn decode_base58_ascii(bench: &mut Bencher) {
    const ALPH: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    test_decode(bench, ALPH);
}

benchmark_group!(benches,
    encode_base2_ascii, encode_base2_utf8, encode_base16_ascii, encode_base16_utf8, encode_base58_ascii, encode_base58_utf8,
    decode_base2_ascii, decode_base2_utf8, decode_base16_ascii, decode_base16_utf8, decode_base58_ascii, decode_base58_utf8
);
benchmark_main!(benches);
