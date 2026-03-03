mod state;

pub use state::ParserState;

#[derive(Debug)]
pub struct ParserIterator<'a> {
    s: &'a str,
    byte_pos: usize,
    state: Vec<Box<dyn ParserState>>,
}

impl<'a> Clone for ParserIterator<'a> {
    fn clone(&self) -> Self {
        Self {
            s: self.s,
            byte_pos: self.byte_pos,
            state: self.state.iter().map(|s| s.clone_box()).collect(),
        }
    }
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
        Self { s, byte_pos: 0, state: vec![] }
    }

    pub fn current_pos(&self) -> usize {
        self.byte_pos
    }

    pub fn is_at_end(&self) -> bool {
        self.byte_pos >= self.s.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.s[self.byte_pos..].chars().next()
    }

    pub fn change_pos(&mut self, byte_pos: usize) {
        assert!(self.s.is_char_boundary(byte_pos));
        self.byte_pos = byte_pos;
    }

    pub fn push_state(&mut self, state: impl ParserState) {
        self.state.push(Box::new(state));
    }

    pub fn push_boxed_state(&mut self, state: Box<dyn ParserState>) {
        self.state.push(state);
    }

    pub fn peek_state(&mut self) -> Option<&Box<dyn ParserState>> {
        self.state.last()
    }

    pub fn check_state<T: ParserState + PartialEq>(&self, expected: &T) -> bool {
        self.state
            .last()
            .and_then(|s| s.as_any().downcast_ref::<T>())
            .map(|s| s == expected)
            .unwrap_or(false)
    }

    pub fn pop_state(&mut self) -> Option<Box<dyn ParserState>> {
        self.state.pop()
    }
}