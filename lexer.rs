// This lexer is VERY simplified for the sake of the simplicity (In this project it is enough sufficient)

use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Some special characters
    Special,

    //Something that cannot be separated
    Word,

    // String literals (not a Word, cause they can be separated like "First part " + " second part")
    Str,

    // Comments will be skipped in obfuscating process
    Comment,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
}

impl Token {
    fn special(content: String) -> Self {
        Self { kind: TokenKind::Special, content }
    }

    fn word(content: String) -> Self {
        Self { kind: TokenKind::Word, content }
    }

    fn string(content: String) -> Self {
        Self { kind: TokenKind::Str, content }
    }

    fn comment(content: String) -> Self {
        Self { kind: TokenKind::Comment, content }
    }
}

pub struct Lexer<Chars: Iterator<Item=char>> {
    chars: Peekable<Chars>
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
    pub fn from_iter(chars: Chars) -> Self {
        Self { chars: chars.peekable() }
    }
}

impl <Chars: Iterator<Item=char>> Iterator for Lexer<Chars> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}

        if let Some(ch) = self.chars.next() {
            let mut content = String::new();
            content.push(ch);

            match ch {
                '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | '&' => Some(Token::special(content)),
                '+' | '*' | '^' => {
                    if let Some(next) = self.chars.peek() {
                       if *next == '=' {
                           content.push(*next);
                           self.chars.next();
                           return Some(Token::special(content));
                       }

                    }
                    
                    return Some(Token::special(content));
                },
                '|' | '&' | ':' => {
                    if let Some(next) = self.chars.peek() {
                       if *next == ch {
                           content.push(*next);
                           self.chars.next();
                           return Some(Token::special(content));
                       }

                    }
                    
                    return Some(Token::special(content));
                },
                '/' => {
                    if let Some(next) = self.chars.peek() {
                        if *next == '/' {
                            self.chars.next();
                            let mut comment_content = String::new();
                            
                            while let Some(x) = self.chars.next_if(|x| *x != '\n') {
                                comment_content.push(x);
                            }

                            self.chars.next();
                            return Some(Token::comment(comment_content));
                        } else if *next == '=' {
                           content.push(*next);
                           self.chars.next();
                           return Some(Token::special(content));
                        }
                    }

                    Some(Token::special(content))
                },
                '"' => {
                    let mut string_content = String::new();
                    while let Some(x) = self.chars.next_if(|x| *x != '"') {
                        string_content.push(x);
                    }
                    self.chars.next();

                    // TODO(#4): unescape string literal
                    Some(Token::string(string_content))
                },
                '=' | '-' => {
                    if let Some(next) = self.chars.peek() {
                        if *next == '>' || *next == '=' {
                            content.push(*next);
                            self.chars.next();

                            return Some(Token::special(content));
                        }
                    }

                    Some(Token::special(content))
                },
                _ => {
                    while let Some(x) = self.chars.next_if(|x| x.is_alphabetic() || *x == '!' || *x == '_' || x.is_digit(10)) {
                        content.push(x);
                    }

                    Some(Token::word(content))
                },
            }
        } else {
            None
        }
    }
}

