# base-x

This is a Rust fork of https://github.com/cryptocoinjs/base-x
And this my very first Rust project: please review the source code!

## Installation

Add this to `Cargo.toml` file:

```
[dependencies]
base-x = "0.1.0"
```

## Usage

```rust
extern crate base_x;

fn main() {
  let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
  let encoded = base_x::encode("01", decoded).unwrap();
  assert_eq!(encoded, "11111111000000001111111100000000");
}

```
