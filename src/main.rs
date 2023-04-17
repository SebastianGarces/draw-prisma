use ::std::fs;

use input_stream::input_stream;
use lexer::tokenizer;

pub mod input_stream;
pub mod lexer;
pub mod parser;

fn main() {
    let file_content = fs::read_to_string("src/input.txt").expect("Unable to read file");
    let stream = input_stream(file_content);

    println!("----Input---- \n{:?}\n ----Input----", stream);
    let _token_stream = tokenizer(stream);

    // println!("{:?}", token_stream);
}
