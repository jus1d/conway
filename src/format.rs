use std::fs;
use std::iter::Peekable;
use lexer::{Token, TokenKind};

mod lexer;

fn print_token(l: &mut Peekable<impl Iterator<Item = Token>>) -> Result<usize, &str> {
    let mut sz = 0;
    if let Some(token) = l.next() {
        match token.kind {
            TokenKind::Word => {
                print!("{}", token.content);
                sz += token.content.len();

                if let Some(next_token) = l.peek() {
                    if next_token.kind == TokenKind::Word {
                        print!(" ");
                        sz += 1;
                    }
                }
            },
            TokenKind::Special => {
                print!("{}", token.content);
                sz += token.content.len();
            },
            TokenKind::Str => {
                print!("\"{}\"", token.content);
                sz += token.content.len() + 2;
            },
            TokenKind::Comment => {},
        }

        return Ok(sz);
    }
    Err("EOF")
}

fn main() {
    let size = 3;
    let pattern = " #   ####";
    let scale_row = 9;
    let scale_col = scale_row * 2;

    let source: String = fs::read_to_string("src/conway-obfuscated.rs").unwrap();
    let mut tokens = lexer::Lexer::from_iter(source.chars()).peekable();

    for row in 0..size {
        let mut i = 0;
        while i < scale_row {
            for col in 0..size {
                let mut j = 0;
                while j < scale_col {
                    if let Some(x) = pattern.chars().nth((size * row) + col) {
                        match x {
                            '#' => {
                                j += print_token(&mut tokens).unwrap_or(scale_col);
                            },
                            _ => {
                                print!(" ");
                                j += 1;
                            },
                        }
                    } else {
                        panic!("ERROR: incorrect string index");
                    }
                }
            }
            println!();
            i += 1;
        }
    }

    println!("\nTotal tokens left: {}", tokens.count());

    // for t in tokens {
    //     match t.kind {
    //         TokenKind::Word => {
    //             print!("{}", t.content);
    //         },
    //        TokenKind::Str => {
    //             print!("\"{}\"", t.content);
    //         },
    //         TokenKind::Special => {
    //             print!("{}", t.content);
    //         },
    //         _ => {},
    //     }
    //   }
}
