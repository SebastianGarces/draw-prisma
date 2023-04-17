pub trait InputStreamTrait {
    fn peek(&self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn eof(&self) -> bool;
    fn croak(&self, msg: &str);
}

#[derive(Debug)]
pub struct InputStream {
    pub input: String,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

impl InputStreamTrait for InputStream {
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.col += 1;
        let next_char = self.input.chars().nth(self.pos);

        if let Some(c) = next_char {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            }
        }

        next_char
    }

    fn eof(&self) -> bool {
        match self.peek() {
            Some(_) => false,
            _ => true,
        }
    }

    fn croak(&self, msg: &str) {
        panic!("{} ({}:{})", msg, self.line, self.col);
    }
}

pub fn input_stream(input: String) -> InputStream {
    InputStream {
        input,
        pos: 0,
        line: 1,
        col: 0,
    }
}
