#[macro_use]
extern crate bencher;
extern crate rand;
extern crate base_x;

use bencher::Bencher;
use base_x::{encode, decode};


fn random_input(size: usize) -> Vec<u8> {
    let mut v = vec![0; size];

    for x in v.iter_mut() {
        *x = rand::random()
    }

    v
}

fn decode_encode(bench: &mut Bencher, alph: &'static str) {
    let input = random_input(100);

    bench.iter(|| {
        let out = encode(alph, &input);
        decode(alph, &out).unwrap()
    })
}

// Actual benchmarks

fn base2(bench: &mut Bencher) {
    const ALPH: &'static str = "01";
    decode_encode(bench, ALPH);
}

fn base16(bench: &mut Bencher) {
    const ALPH: &'static str = "0123456789abcdef";
    decode_encode(bench, ALPH);
}

fn base58(bench: &mut Bencher) {
    const ALPH: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    decode_encode(bench, ALPH);
}

benchmark_group!(benches, base2, base16, base58);
benchmark_main!(benches);
