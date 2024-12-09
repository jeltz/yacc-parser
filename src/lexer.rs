use crate::token::Spanned;
use crate::token::Token;
use core::iter::Peekable;
use core::str::CharIndices;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
    percent_percent_count: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.curr_pos();
        let token = 'outer: loop {
            match self.chars.next()?.1 {
                // '<char>'
                '\'' => {
                    if self.chars.next().is_none() {
                        break Token::Err;
                    }
                    if let Some((_, '\'')) = self.chars.next() {
                        break Token::Char;
                    }
                    break Token::Err;
                }
                '/' => match self.chars.next()?.1 {
                    '/' => {
                        for (_, c) in self.chars.by_ref() {
                            if c == '\n' {
                                break;
                            }
                        }
                        start = self.curr_pos();
                        continue;
                    }
                    '*' => {
                        loop {
                            match self.chars.next() {
                                Some((_, '*')) => match self.chars.peek() {
                                    Some((_, '/')) => {
                                        self.chars.next();
                                        break;
                                    }
                                    Some(_) => {}
                                    None => {
                                        break 'outer Token::Err;
                                    }
                                },
                                Some(_) => {}
                                None => {
                                    break 'outer Token::Err;
                                }
                            }
                        }
                        start = self.curr_pos();
                        continue;
                    }
                    _ => break Token::Err,
                },
                '\n' | ' ' | '\t' => {
                    start = self.curr_pos();
                    continue;
                }
                '=' => {
                    break Token::Equal;
                }
                '0'..='9' => {
                    while let Some((_, '0'..='9')) = self.chars.peek() {
                        self.chars.next();
                    }
                    break Token::Number;
                }
                '"' => {
                    break loop {
                        match self.chars.next() {
                            Some((_, '"')) => {
                                break Token::String;
                            }
                            Some(_) => {}
                            None => {
                                break Token::Err;
                            }
                        }
                    }
                }
                '%' => match self.chars.next()?.1 {
                    '%' => {
                        self.percent_percent_count += 1;
                        if self.percent_percent_count >= 2 {
                            for _ in self.chars.by_ref() {}
                            break Token::Epilogue;
                        }
                        break Token::PercentPercent;
                    }
                    '{' => {
                        break loop {
                            match self.chars.next() {
                                Some((_, '%')) => match self.chars.peek() {
                                    Some((_, '}')) => {
                                        self.chars.next();
                                        break Token::Prologue;
                                    }
                                    Some(_) => {}
                                    None => {
                                        break Token::Err;
                                    }
                                },
                                Some(_) => {}
                                None => {
                                    break Token::Err;
                                }
                            }
                        }
                    }
                    'a'..='z' | 'A'..='Z' => {
                        while let Some((_, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-')) =
                            self.chars.peek()
                        {
                            self.chars.next();
                        }
                        break Token::Directive;
                    }
                    _ => {
                        break Token::Err;
                    }
                },
                '|' => {
                    break Token::Bar;
                }
                ':' => {
                    break Token::Colon;
                }
                ';' => {
                    break Token::SemiColon;
                }
                // {...}
                '{' => {
                    let mut depth = 1;
                    break loop {
                        match self.chars.next() {
                            Some((_, '{')) => depth += 1,
                            Some((_, '}')) => {
                                depth -= 1;
                                if depth == 0 {
                                    break Token::Code;
                                }
                            }
                            Some(_) => {}
                            None => {
                                break Token::Err;
                            }
                        }
                    };
                }
                'a'..='z' | 'A'..='Z' => {
                    while let Some((_, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')) = self.chars.peek()
                    {
                        self.chars.next();
                    }
                    break Token::Ident;
                }
                '<' => {
                    break loop {
                        match self.chars.next() {
                            Some((_, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')) => {}
                            Some((_, '>')) => break Token::Type,
                            _ => break Token::Err,
                        }
                    };
                }
                _ => {
                    break Token::Err;
                }
            };
        };
        Some(Spanned::new(token, start..self.curr_pos()))
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            percent_percent_count: 0,
        }
    }

    fn curr_pos(&mut self) -> usize {
        self.chars.peek().map_or(self.input.len(), |c| c.0)
    }
}
