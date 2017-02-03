use std::collections::HashMap;

const INVALID_INDEX: u8 = 0xFF;

pub trait Alphabet {
    type Lookup: CharLookup;

    /// Get a character from Alphabet at index.
    ///
    /// This method can panic if usize doesn't fit in base.
    fn get(&self, usize) -> char;

    /// Get a byte array of the alphabet. This can be useful
    /// for ASCII-based alphabets.
    fn as_bytes(&self) -> &[u8];

    /// Returns a lookup type used to find an index of a char
    /// in the Alphabet.
    fn lookup_table(&self) -> Self::Lookup;

    /// Get the base (length in characters) of the Alphabet.
    fn base(&self) -> usize;
}

pub trait CharLookup: Sized {
    /// Get the index of the `char` in the Alphabet. If `char`
    /// is not in the Alphabet return `None`.
    fn get(&self, char) -> Option<usize>;
}

impl<'a> Alphabet for &'a [u8] {
    type Lookup = [u8; 256];

    #[inline(always)]
    fn get(&self, index: usize) -> char {
        self[index] as char
    }

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        *self
    }

    /// Produces the lookup table matching byte index [0 - 255] to a
    /// corresponding alphabet byte.
    ///
    /// The default implementation will produce the lookup table on
    /// runtime, and recalculate it every time encoding is invoked.
    /// Ideally a custom implementation of the `Alphabet` would return
    /// a `&'static` precalculated table here.
    #[inline(always)]
    fn lookup_table(&self) -> Self::Lookup {
        let mut lookup = [INVALID_INDEX; 256];

        for (i, byte) in self.as_bytes().iter().enumerate() {
            lookup[*byte as usize] = i as u8;
        }

        lookup
    }

    #[inline(always)]
    fn base(&self) -> usize {
        self.len()
    }
}

impl<'a> Alphabet for &'a str {
    type Lookup = HashMap<char, usize>;

    #[inline(always)]
    fn get(&self, index: usize) -> char {
        self.chars().nth(index).expect("Index will be % base, ergo in alphabet range; qed")
    }

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    /// Produces the hashmap matching any `char` to it's index in alphabet.
    #[inline(always)]
    fn lookup_table(&self) -> Self::Lookup {
        // this is byte-length, which might or might not be enough,
        // we might suffer a reallocation at some point.
        let mut map = HashMap::with_capacity(self.len());

        for (index, ch) in self.chars().enumerate() {
            map.insert(ch, index);
        }

        map
    }

    #[inline(always)]
    fn base(&self) -> usize {
        self.chars().count()
    }
}

impl CharLookup for [u8; 256] {
    #[inline(always)]
    fn get(&self, byte: char) -> Option<usize> {
        match self[byte as u8 as usize] {
            INVALID_INDEX => None,
            byte => Some(byte as usize)
        }
    }
}

impl<'a> CharLookup for &'a [u8; 256] {
    #[inline(always)]
    fn get(&self, byte: char) -> Option<usize> {
        match self[byte as u8 as usize] {
            INVALID_INDEX => None,
            byte => Some(byte as usize)
        }
    }
}

impl CharLookup for HashMap<char, usize> {
    #[inline(always)]
    fn get(&self, ch: char) -> Option<usize> {
        self.get(&ch).map(|index| *index)
    }
}

#[cfg(test)]
mod test {
    use super::{Alphabet, CharLookup};
    use std::collections::HashMap;

    #[test]
    fn lookup_str() {
        let alphabet = "abcd";

        let lookup: HashMap<char, usize> = alphabet.lookup_table();

        assert_eq!(CharLookup::get(&lookup, 'a'), Some(0));
        assert_eq!(CharLookup::get(&lookup, 'b'), Some(1));
        assert_eq!(CharLookup::get(&lookup, 'c'), Some(2));
        assert_eq!(CharLookup::get(&lookup, 'd'), Some(3));
        assert_eq!(CharLookup::get(&lookup, 'e'), None);
        assert_eq!(CharLookup::get(&lookup, '7'), None);
        assert_eq!(CharLookup::get(&lookup, '$'), None);
    }

    #[test]
    fn lookup_bytes() {
        let alphabet: &[u8] = b"qwer";

        let lookup: [u8; 256] = alphabet.lookup_table();

        assert_eq!(lookup.get('q'), Some(0));
        assert_eq!(lookup.get('w'), Some(1));
        assert_eq!(lookup.get('e'), Some(2));
        assert_eq!(lookup.get('r'), Some(3));
        assert_eq!(lookup.get('t'), None);
        assert_eq!(lookup.get('*'), None);
        assert_eq!(lookup.get('_'), None);
    }
}
