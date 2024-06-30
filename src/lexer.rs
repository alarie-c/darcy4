use std::iter::Peekable;

use crate::token::Token;

pub struct Lexer<Iter: Iterator<Item = char>> {
    pub stream: Peekable<Iter>,
    pub output: Vec<Token>,
    cursor: usize,
    ch: char,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {

    // creates a new lexer and advances to the first character
    pub fn new(stream: Peekable<Iter>) -> Option<Self> {
        let mut lexer = Self {
            stream,
            output: Vec::<Token>::new(),
            cursor: 0,
            ch: '.',
        };

        if let Some(character) = lexer.stream.next() {
            lexer.ch = character;
            Some(lexer)
        } else {
            None
        }
    }

    pub fn scan(&mut self) {
        loop {
            self.tokenize();
            if self.check() { continue; } else { break; }
        }

        self.push(Token::Eof { offset: self.cursor });
    }

    fn next(&mut self) -> Option<char> {
        if let Some(character) = self.stream.next() {
            self.ch = character;
            self.cursor += 1;
            Some(character)
        } else {
            None
        }
    }

    fn check(&mut self) -> bool {
        if let Some(_) = self.next() {
            true
        } else {
            false
        }
    }

    fn push(&mut self, token: Token) {
        self.output.push(token)
    }

    fn take_alphanum(&mut self) -> String {
        let mut buffer = String::from(self.ch);
        
        loop {
            if let Some(character) = self.next() {
                if character.is_alphanumeric() || character == '_' {
                    buffer.push(character);
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        buffer
    }

    fn take_number(&mut self) -> String {
        let mut buffer = String::from(self.ch);
        
        loop {
            if let Some(character) = self.next() {
                if character.is_numeric() {
                    buffer.push(character);
                    continue;
                } else if character == '_' {
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        buffer
    }

    fn take_literal(&mut self) -> Result<String, String> {
        let mut buffer = String::new();
        
        loop {
            if let Some(character) = self.next() {
                if character == '"' {
                    return Ok(buffer);
                } else {
                    buffer.push(character);
                    continue;
                }
            } else {
                return Err(buffer);
            }
        }
    }

    fn tokenize(&mut self) {
        match self.ch {
            '(' => self.push(Token::LParen { offset: self.cursor }),
            ')' => self.push(Token::RParen { offset: self.cursor }),
            ';' => self.push(Token::SemiColon { offset: self.cursor }),
            '+' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::PlusEqual { offset: self.cursor - 1 })
                    } else if c == '+' {
                        self.push(Token::PlusPlus { offset: self.cursor - 1 })
                    } else {
                        self.push(Token::Plus { offset: self.cursor - 1 });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Plus { offset: self.cursor })
                }
            },
            '=' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::EqualEqual { offset: self.cursor - 1 })
                    } else {
                        self.push(Token::Equal { offset: self.cursor - 1 });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Equal { offset: self.cursor })
                }
            },

            // number literals
            '0'..='9' => {
                let number = self.take_number();
                self.push(Token::Number { offset: self.cursor, value: number })
            }

            // string literal
            '"' => {
                match self.take_literal() {
                    Ok(literal) => self.push(Token::String { offset: self.cursor, value: literal }),
                    Err(_) => panic!("[DY4] Non terminating literal!"),
                }
            }

            // identifers & keywords
            'a'..='z' | 'A'..='Z' => {
                let alphanum = self.take_alphanum();
                
                // match keywords
                match alphanum.as_str() {
                    "return" => self.push(Token::Return { offset: self.cursor }),
                    "fn" => self.push(Token::Fn { offset: self.cursor }),

                    // identifers
                    _ => self.push(Token::Indent { offset: self.cursor, value: alphanum }),
                }

                self.tokenize();
            }
            _ => {},
        }
    }
}