use std::iter::Peekable;

use crate::token::Token;

// takes a stream of chars and creates tokenized output
pub struct Lexer<Iter: Iterator<Item = char>> {
    pub stream: Peekable<Iter>,
    pub output: Vec<Token>,
    cursor: usize,
    offset: usize,
    ch: char,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {
    // creates a new lexer and advances to the first character
    pub fn new(stream: Peekable<Iter>) -> Option<Self> {
        let mut lexer = Self {
            stream,
            output: Vec::<Token>::new(),
            offset: 0,
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

    // scans each char and tokenizes it
    pub fn scan(&mut self) {
        loop {
            self.tokenize();
            if self.check() {
                continue;
            } else {
                break;
            }
        }

        self.push(Token::Eof {
            offset: self.cursor,
        });
    }

    // advances the iterator and increments cursor and char values
    fn next(&mut self) -> Option<char> {
        if let Some(character) = self.stream.next() {
            self.ch = character;
            self.cursor += 1;
            Some(character)
        } else {
            None
        }
    }

    // advances the iterator and returns if it was successful or not
    fn check(&mut self) -> bool {
        if let Some(_) = self.next() {
            true
        } else {
            false
        }
    }

    // pushes a given token to the output
    fn push(&mut self, token: Token) {
        self.output.push(token)
    }

    // pushes chars to a buffer until a symbol is reached
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

    // pushes numbers to a buffer until a non-number is found
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

    // takes any char until another " is found
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

    // matches the current char to a token and pushes it to the output
    fn tokenize(&mut self) {
        self.offset = self.cursor;

        match self.ch {
            '(' => self.push(Token::LParen {
                offset: self.offset,
            }),
            ')' => self.push(Token::RParen {
                offset: self.offset,
            }),
            '{' => self.push(Token::LCurl {
                offset: self.offset,
            }),
            '}' => self.push(Token::RCurl {
                offset: self.offset,
            }),
            '[' => self.push(Token::LBrac {
                offset: self.offset,
            }),
            ']' => self.push(Token::RBrac {
                offset: self.offset,
            }),
            ';' => self.push(Token::SemiColon {
                offset: self.offset,
            }),
            '#' => self.push(Token::Hash {
                offset: self.offset,
            }),
            '%' => self.push(Token::Modulo {
                offset: self.offset,
            }),
            '*' => self.push(Token::Star {
                offset: self.offset,
            }),
            ':' => self.push(Token::Colon {
                offset: self.offset,
            }),
            '+' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::PlusEqual {
                            offset: self.offset,
                        })
                    } else if c == '+' {
                        self.push(Token::PlusPlus {
                            offset: self.offset,
                        })
                    } else {
                        self.push(Token::Plus {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Plus {
                        offset: self.offset,
                    })
                }
            }
            '-' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::MinusEqual {
                            offset: self.offset,
                        });
                    } else if c == '-' {
                        self.push(Token::MinusMinus {
                            offset: self.offset,
                        });
                    } else if c == '>' {
                        self.push(Token::Arrow {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::Minus {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Minus {
                        offset: self.offset,
                    });
                }
            }
            '/' => {
                if let Some(c) = self.next() {
                    if c == '/' {
                        self.push(Token::SlashSlash {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::Slash {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Slash {
                        offset: self.offset,
                    });
                }
            }
            '=' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::EqualEqual {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::Equal {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Equal {
                        offset: self.offset,
                    });
                }
            }
            '<' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::LessEqual {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::Less {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Less {
                        offset: self.offset,
                    });
                }
            }
            '!' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::BangEqual {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::Bang {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::Bang {
                        offset: self.offset,
                    });
                }
            }
            '>' => {
                if let Some(c) = self.next() {
                    if c == '=' {
                        self.push(Token::MoreEqual {
                            offset: self.offset,
                        });
                    } else if c == '>' {
                        self.push(Token::DoubleArrow {
                            offset: self.offset,
                        });
                    } else {
                        self.push(Token::More {
                            offset: self.offset,
                        });
                        self.tokenize();
                    }
                } else {
                    self.push(Token::More {
                        offset: self.offset,
                    });
                }
            }

            // number literals
            '0'..='9' => {
                let number = self.take_number();
                self.push(Token::Number {
                    offset: self.offset,
                    value: number,
                })
            }

            // string literal
            '"' => match self.take_literal() {
                Ok(literal) => self.push(Token::String {
                    offset: self.offset,
                    value: literal,
                }),
                Err(_) => panic!("[DY4] Non terminating literal!"),
            },

            // identifers & keywords
            'a'..='z' | 'A'..='Z' => {
                let alphanum = self.take_alphanum();

                // match keywords
                match alphanum.as_str() {
                    "return" => self.push(Token::Return {
                        offset: self.offset,
                    }),
                    "func" => self.push(Token::Func {
                        offset: self.offset,
                    }),
                    "if" => self.push(Token::If {
                        offset: self.offset,
                    }),
                    "else" => self.push(Token::Else {
                        offset: self.offset,
                    }),
                    "elif" => self.push(Token::Elif {
                        offset: self.offset,
                    }),

                    // identifers
                    _ => self.push(Token::Indent {
                        offset: self.offset,
                        value: alphanum,
                    }),
                }

                self.tokenize();
            }
            _ => {}
        }
    }
}
