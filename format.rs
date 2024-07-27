use std::fs;
use std::process::exit;
use lexer::{Token, TokenKind};

mod lexer;

fn main() {
    let size = 3;
    let pattern = " #   ####";
    let scale_row = 11;
    let scale_col = scale_row * 2;
    let blacks = pattern.chars().filter(|ch| *ch == '#').count();

    let source: String = fs::read_to_string("./conway-obfuscated.rs").unwrap();
    let mut tokens = lexer::Lexer::from_iter(source.chars()).peekable();

    let mut col = 0;
    let mut row = 0;
    
    for row in 0..size {
        let mut i = 0;
        while i < scale_row {
            for col in 0..size {
                let mut j = 0;
                while j < scale_col {
                    if let Some(x) = pattern.chars().nth((size * row) + col) {
                        match x {
                            '#' => {
                                match tokens.next() {
                                    Some(tok) => {
                                        match tok.kind {
                                            TokenKind::Word => {
                                                print!("{}", tok.content);
                                                match tokens.peek() {
                                                    Some(next_tok) => {
                                                        if next_tok.kind == TokenKind::Word {
                                                            print!(" ");
                                                            j += 1;
                                                        }
                                                    },
                                                    None => {},
                                                }
                                            },
                                            TokenKind::Special => {
                                                print!("{}", tok.content);
                                            },
                                            TokenKind::Str => {
                                                print!("\"{}\"", tok.content);
                                            },
                                            _ => {},
                                        }
                                        j += tok.content.len();
                                    },
                                    None => {
                                        exit(1);
                                    }
                                }
                            },
                            _ => {
                                print!(" ");
                                j += 1;
                            },
                        }
                    } else {
                        println!("ERROR: incorrect string index");
                        exit(1);
                    }
                }
            }
            println!();
            i += 1;
        }
    }

    println!();
    for t in tokens {
        match t.kind {
            TokenKind::Word => {
                print!("{}", t.content);
            },
            TokenKind::Str => {
                print!("\"{}\"", t.content);
            },
            TokenKind::Special => {
                print!("{}", t.content);
            },
            _ => {},
        }
    }
}
