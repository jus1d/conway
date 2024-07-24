use std::fs;
use std::fs::{File};
use std::io::Write;

mod lexer;

fn main() {
    let filepath = String::from("./conway.rs");
    let source: String = fs::read_to_string(filepath.clone()).unwrap();

    let tokens = lexer::Lexer::from_iter(source.chars());
    
    let mut output = File::create("output.rs").expect("creation failed");
    for token in tokens {
        println!("{:?}", token);

        use lexer::Token::{*};
        match token {
            Word(word) => {
                output.write(word.as_bytes());
                output.write(" ".as_bytes());
            },
            Special(tok) => {
                output.write(tok.as_bytes());
            },
            Str(content) => {
                output.write("\"".as_bytes());
                output.write(content.as_bytes());
                output.write("\"".as_bytes());
            },
            _ => {},
        }
    }
}
