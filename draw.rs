use std::fs;
use std::process::exit;
use lexer::{Token, Token::*};

mod lexer;

fn main() {
    let size = 3;
    let pattern = " #   ####";
    let scale_row = 14;
    let scale_col = scale_row * 2;
    let blacks = pattern.chars().filter(|ch| *ch == '#').count();

    let source: String = fs::read_to_string("./conway.rs").unwrap();
    let mut tokens = lexer::Lexer::from_iter(source.chars());

    println!("{:?}", tokens.next());

    let mut col = 0;
    let mut row = 0;
    
    for row in 0..size {
        let mut i = 0;
        while i < scale_row {
            for col in 0..size {
                let mut j = 0;
                while j < scale_col {
                    if let Some(x) = pattern.chars().nth((size * row) + col) {
                        print!("{x}");
                    } else {
                        println!("ERROR: incorrect string index");
                        exit(1);
                    }
                    j += 1;
                }
            }
            println!();
            i += 1;
        }
    }
}
