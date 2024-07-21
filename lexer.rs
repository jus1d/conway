use std::iter::Peekable;
use std::fs;

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Keyword(String),
    Identifier(String),
    Str(String),
}

fn is_keyword(word: String) -> bool {
    match word.as_str() {
        "fn" => true,
        _ => false,
    }
}

struct Lexer<Chars: Iterator<Item=char>> {
    chars: Peekable<Chars>
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
    fn from_iter(chars: Chars) -> Self {
        Self { chars: chars.peekable() }
    }
}

impl <Chars: Iterator<Item=char>> Iterator for Lexer<Chars> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Token> {
        while let Some(_) = self.chars.next_if(|s| s.is_whitespace()) {}
        
        if let Some(x) = self.chars.next() {
            let mut text = String::new();
            text.push(x);
            match x {
                '(' => Some(Token::OpenParen),
                ')' => Some(Token::CloseParen),
                '{' => Some(Token::OpenCurly),
                '}' => Some(Token::CloseCurly),
                '[' => Some(Token::OpenBracket),
                ']' => Some(Token::CloseBracket),
                ';' => Some(Token::Semicolon),
                ':' => Some(Token::Colon),
                '"' => {
                    let mut content = String::new();
                    while let Some(ch) = self.chars.next() {
                        if ch == '"' { 
                            return Some(Token::Str(content));
                        }
                        content.push(ch);
                    }
                    todo!("Report unclosed string literal");
                },
                _ => {
                    while let Some(x) = self.chars.next_if(|x| x.is_alphabetic() || *x == '!' || *x == '_') {
                        text.push(x);
                    }
                    
                    if is_keyword(text.clone()) {
                        return Some(Token::Keyword(text));
                    } else {
                       return Some(Token::Identifier(text));
                    }
                },
            }
        } else {
            None
        }
    }
}

fn main() {
    let filepath: String = String::from("./hello.rs");
    let source = fs::read_to_string(filepath.clone()).unwrap();

    let lexer = Lexer::from_iter(source.chars());
    
    for token in lexer {
        println!("{:?}", token);
    }
}
