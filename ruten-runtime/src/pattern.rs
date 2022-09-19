pub struct Pattern {
    bytes: Vec<Option<u8>>
}

impl Pattern {
    pub fn new(bytes: Vec<Option<u8>>) -> Self {
        Self { bytes }
    }

    pub fn from_str(pattern: &str) -> Self {
        let mut bytes = Vec::new();

        for character in pattern.split(" ").into_iter() {
            if let Ok(byte) = u8::from_str_radix(character, 16) {
                bytes.push(Some(byte));
            } else {
                bytes.push(None);
            }
        }

        Self::new(bytes)
    }

    pub fn bytes(&self) -> Vec<Option<u8>> {
        self.bytes.clone()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl PartialEq<&[u8]> for Pattern {
    fn eq(&self, bytes: &&[u8]) -> bool {
        if self.bytes.len() != bytes.len() {
            return false;
        }

        for i in 0..bytes.len() {
            if let Some(byte) = self.bytes[i] {
                if byte != bytes[i] {
                    return false;
                }
            }
        }

        true
    }
}

impl PartialEq<Pattern> for &[u8] {
    fn eq(&self, bytes: &Pattern) -> bool {
        bytes == self
    }
}