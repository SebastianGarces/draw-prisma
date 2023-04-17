use crate::input_stream::{InputStream, InputStreamTrait};
use regex::Regex;

pub trait Tokenizer {
    fn peek(&self) -> Option<Token>;
    fn next(&mut self) -> Option<Token>;
    fn eof(&self) -> bool;
    fn croak(&self, msg: &str);
}
#[derive(Debug)]
pub struct TokenStream {
    pub stream: InputStream,
}

impl Tokenizer for TokenStream {
    fn peek(&self) -> Option<Token> {
        // needs implementation
        Some(Token {
            token_type: TokenType::Keyword,
            value: String::from("a"),
        })
    }

    fn next(&mut self) -> Option<Token> {
        // needs implementation
        Some(Token {
            token_type: TokenType::Keyword,
            value: String::from("a"),
        })
    }

    fn eof(&self) -> bool {
        match self.peek() {
            Some(_) => false,
            _ => true,
        }
    }

    fn croak(&self, msg: &str) {
        self.stream.croak(msg);
    }
}

#[derive(Debug)]
enum TokenType {
    Punctuation,
    Keyword,
    Variable,
    Modifier,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

pub fn tokenizer(input: InputStream) -> TokenStream {
    let mut stream = input;
    let _token = String::new();
    let mut current_char = stream.peek();

    println!("\n----------Output----------\n");
    while !stream.eof() {
        if let Some(c) = current_char {
            if is_comment(&mut stream) {
                skip_comment(&mut stream);
                current_char = stream.next();
                continue;
            }

            if c.is_whitespace() {
                current_char = stream.next();
                continue;
            }

            print!("{}", c);
            current_char = stream.next();
        }
    }
    println!("\n\n----------Output----------\n\n");

    TokenStream { stream }
}

fn is_comment(stream: &mut InputStream) -> bool {
    let c = stream.peek();

    match c {
        Some('/') => {
            let next_c = stream.next();
            match next_c {
                Some('/') => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn skip_comment(stream: &mut InputStream) {
    read_while(stream, |c| c != '\n');
}

fn read_while(stream: &mut InputStream, predicate: fn(char) -> bool) -> String {
    let mut str = String::new();

    while let Some(c) = stream.peek() {
        if predicate(c) {
            str.push(c);
            stream.next();
        } else {
            break;
        }
    }

    str
}
