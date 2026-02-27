#[derive(Debug, Clone)]
pub struct ParserIterator<'a> {
    s: &'a str,
    byte_pos: usize,
}

impl Iterator for ParserIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.s[self.byte_pos..].chars().next()?;
        self.byte_pos += ch.len_utf8();
        Some(ch)
    }
}

impl<'a> From<&'a str> for ParserIterator<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

impl<'a> ParserIterator<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, byte_pos: 0 }
    }

    pub fn current_pos(&self) -> usize {
        self.byte_pos
    }

    pub fn peek(&self) -> Option<char> {
        self.s[self.byte_pos..].chars().next()
    }

    pub fn change_pos(&mut self, byte_pos: usize) {
        assert!(self.s.is_char_boundary(byte_pos));
        self.byte_pos = byte_pos;
    }
}