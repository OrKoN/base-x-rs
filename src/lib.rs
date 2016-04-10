use std::collections::HashMap;

pub fn encode(alphabet: &str, input: Vec<i16>) -> String {
  if input.len() == 0 {
    return String::new();
  }

  let base = alphabet.len() as i16;
  let mut alphabet_map = HashMap::with_capacity(alphabet.len());

  for (i, c) in alphabet.chars().enumerate() {
    alphabet_map.insert(i, c);
  }

  let mut digits: Vec<i16> = vec![0];
  for c in &input {
    let mut j = 0;
    let mut carry = *c;
    while j < digits.len() {
      carry = carry + (digits[j] << 8);
      digits[j] = carry % base;
      carry = (carry / base) | 0;
      j+=1;
    }

    while carry > 0 {
      digits.push(carry % base);
      carry = (carry / base) | 0;
    }
  }

  let mut k = 0;
  while k < input.len()-1 {
    if input[k] == 0 {
      digits.push(0);
      k = k + 1;
    } else {
      break;
    }
  }

  let digits_len = digits.len();
  let mut output = String::with_capacity(digits_len);
  for i in (0..digits_len).rev() {
    let chr = alphabet_map.get(&(digits[i] as usize)).unwrap();
    output.push(*chr);
  }

  return output;
}

pub fn decode(alphabet: &str, input: &str) -> Vec<i16> {
  if input.len() == 0 {
    return vec![];
  }

  let base = alphabet.len() as i16;
  let leader = alphabet.chars().nth(0).unwrap();
  let mut alphabet_map = HashMap::new();

  for (i, c) in alphabet.chars().enumerate() {
    alphabet_map.insert(c, i as i16);
  }

  let mut bytes = vec![0];
  for c in input.chars() {
    let mut carry = alphabet_map.get(&c).unwrap().clone();
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

  let mut k = 0;
  while k < input.len()-1 {
    if input.chars().nth(k).unwrap() == leader {
      bytes.push(0);
      k = k + 1;
    } else {
      break;
    }
  }

  bytes.reverse();
  bytes
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
      for  value in valid {
        let obj = value.as_object().unwrap();
        let alphabet_name = obj.get("alphabet").unwrap().to_string();
        let alphabet_name: String = json::decode(&alphabet_name).unwrap();
        let input = obj.get("string").unwrap().to_string();
        let input: String = json::decode(&input).unwrap();
        let alphabet = alphabets.get(&alphabet_name).unwrap().to_string();
        let alphabet: String = json::decode(&alphabet).unwrap();
        let decoded = decode(&alphabet, &input);
        let encoded = encode(&alphabet, decoded);
        assert_eq!(encoded, input);
      }
    }
}
