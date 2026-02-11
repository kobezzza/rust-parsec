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

    pub fn prev(&mut self) -> Option<char> {
        if self.byte_pos == 0 {
            return None;
        }

        let mut prev_byte_pos = self.byte_pos - 1;

        while prev_byte_pos > 0 && !self.s.is_char_boundary(prev_byte_pos) {
            prev_byte_pos -= 1;
        }

        let ch = self.s[prev_byte_pos..].chars().next()?;
        self.byte_pos = prev_byte_pos;

        Some(ch)
    }

    pub fn rewind(&mut self, n: usize) -> Option<char> {
        let mut ch = None;

        for _ in 0..n {
            let prev = self.prev();

            if prev.is_some() {
                ch = prev;

            } else {
                break;
            }
        }

        ch
    }
}