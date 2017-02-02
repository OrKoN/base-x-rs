pub const INVALID_INDEX: u8 = 0xFF;

pub trait Alphabet {
    fn as_bytes(&self) -> &[u8];

    /// Produces the lookup table matching byte index [0 - 255] to a
    /// corresponding alphabet byte.
    ///
    /// The default implementation will produce the lookup table on
    /// runtime, and recalculate it every time encoding is invoked.
    /// Ideally a custom implementation of the `Alphabet` would return
    /// a `&'static` precalculated table here.
    #[inline]
    fn lookup_table(&self) -> [u8; 256] {
        let mut lookup = [INVALID_INDEX; 256];

        for (i, byte) in self.as_bytes().iter().enumerate() {
            lookup[*byte as usize] = i as u8;
        }

        lookup
    }

    #[inline]
    fn base(&self) -> usize {
        self.as_bytes().len()
    }
}

impl<T: AsRef<[u8]>> Alphabet for T {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::Alphabet;
    use super::INVALID_INDEX;

    #[test]
    fn lookup_str() {
        let alphabet = "abcd";

        let lookup = alphabet.lookup_table();

        assert_eq!(lookup[b'a' as usize], 0);
        assert_eq!(lookup[b'b' as usize], 1);
        assert_eq!(lookup[b'c' as usize], 2);
        assert_eq!(lookup[b'd' as usize], 3);
        assert_eq!(lookup[b'e' as usize], INVALID_INDEX);
        assert_eq!(lookup[b'$' as usize], INVALID_INDEX);
        assert_eq!(lookup[b'7' as usize], INVALID_INDEX);
    }

    #[test]
    fn lookup_bytes() {
        let alphabet: &[u8] = &[0x13,0x37,0xDE,0xAD];

        let lookup = alphabet.lookup_table();

        assert_eq!(lookup[0x13], 0);
        assert_eq!(lookup[0x37], 1);
        assert_eq!(lookup[0xDE], 2);
        assert_eq!(lookup[0xAD], 3);
        assert_eq!(lookup[0x00], INVALID_INDEX);
        assert_eq!(lookup[0x80], INVALID_INDEX);
        assert_eq!(lookup[0xFF], INVALID_INDEX);
    }
}
