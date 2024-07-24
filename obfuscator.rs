use std::fs;
use std::fs::{File};
use std::io::Write;
use lexer::{Token, Token::*};

mod lexer;

fn main() {
    let source: String = fs::read_to_string("./conway.rs").unwrap();
    let tokens: Vec<Token> = lexer::Lexer::from_iter(source.chars()).collect();

    let mut output = File::create("output.rs").expect("creation failed");
    
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Word(word) => {
                output.write(word.as_bytes());
                if i != tokens.len() {
                    if let Word(_) = &tokens[i+1] {
                        output.write(" ".as_bytes());
                    }
                }
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
        i += 1;
    }
}
