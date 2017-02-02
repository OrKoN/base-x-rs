# base-x

This is a Rust fork of https://github.com/cryptocoinjs/base-x
And this my very first Rust project: please review the source code!

## Installation

Add this to `Cargo.toml` file:

```
[dependencies]
base-x = "0.2.0"
```

## Usage

```rust
extern crate base_x;

fn main() {
  let alphabet = "01";
  let decoded = base_x::decode(alphabet.as_bytes(), "11111111000000001111111100000000").unwrap();
  let encoded = base_x::encode(alphabet.as_bytes(), decoded).unwrap();
  assert_eq!(encoded, "11111111000000001111111100000000");
}

```

## Changelog

- 0.2.0

  Breaking change: alphabet has to be provided as an array of bytes instead of a string.

- 0.1.0

  initial version

## Contributors

- [Friedel Ziegelmayer](https://github.com/dignifiedquire)
- [Maciej Hirsz](https://github.com/maciejhirsz)