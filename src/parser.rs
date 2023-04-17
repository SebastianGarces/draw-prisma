use std::str::FromStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum States {
    Idle,
    InDefinition,
}

#[derive(Debug, PartialEq, EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
enum Definitions {
    Model,
    Generator,
    Datasource,
    Client,
    Provider,
}

#[derive(Debug)]
enum TokenType {
    Definition(Definitions),
    Value(String),
    SpecialCharacter(char),
}

trait Parse {
    fn parse(&mut self);
    fn get_next_token(&mut self) -> String;
}

struct Parser {
    string: String,
    cursor: usize,
}

impl Parse for Parser {
    fn parse(&mut self) {
        let state = States::Idle;

        while self.cursor < self.string.len() {
            let token = self.get_next_token();
            println!("token: {}, state: {:?}", token, state);

            let token_type = get_token_type(&token).expect("Invalid token type");

            match &token_type {
                TokenType::Value(value) => {
                    println!("value: {}", value);
                }
                TokenType::SpecialCharacter(c) => {
                    println!("special character: {}", c);
                }
                TokenType::Definition(definition) => {
                    println!("definition: {:?}", definition);
                }
            }

            println!("token_type: {:?}", token_type);
        }
    }

    fn get_next_token(&mut self) -> String {
        let mut token = String::new();

        while self.cursor < self.string.len() {
            let current_char = self.string.chars().nth(self.cursor).unwrap();
            let is_space = current_char == ' ';
            let is_new_line = current_char == '\n' || current_char == '\r' || current_char == '\t';
            let is_no_op = is_space && token.len() == 0 || is_new_line && token.len() == 0;

            if is_no_op {
                self.cursor += 1;
                continue;
            }

            if is_space || is_new_line {
                self.cursor += 1;
                break;
            }

            token.push(current_char);
            self.cursor += 1;
        }

        token
    }
}

fn is_value(token: &String) -> bool {
    let keywords = vec![
        "model",
        "generator",
        "datasource",
        "client",
        "main",
        "provider",
    ];

    keywords
        .iter()
        .all(|&keyword| keyword != token.to_lowercase())
}

fn is_definition(token: &String) -> bool {
    let keywords = vec![
        "model",
        "generator",
        "datasource",
        "client",
        "main",
        "provider",
    ];

    keywords
        .iter()
        .any(|&keyword| keyword == token.to_lowercase())
}

fn is_special_character(token: &String) -> bool {
    let allowed_chars = vec!['{', '}', '(', ')', '[', ']', '"', '@', ',', '?', '='];

    allowed_chars
        .iter()
        .any(|&c| c == token.chars().nth(0).unwrap_or('~'))
}

#[derive(Debug)]
struct InvalidTokenType;
fn get_token_type(token: &String) -> Result<TokenType, InvalidTokenType> {
    if token.len() == 1 && is_special_character(token) {
        return Ok(TokenType::SpecialCharacter(token.chars().nth(0).unwrap()));
    } else if is_value(token) {
        Ok(TokenType::Value(token.to_string()))
    } else if is_definition(token) {
        let definition = get_definition(token);
        Ok(TokenType::Definition(definition))
    } else {
        Err(InvalidTokenType)
    }
}

fn get_definition(token: &String) -> Definitions {
    Definitions::from_str(&token).expect(&format!("Token {} is not a valid definition.", token))
}
