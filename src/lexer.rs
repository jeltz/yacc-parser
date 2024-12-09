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
        let token = loop {
            match self.chars.next()?.1 {
                // '<char>'
                '\'' => {
                    self.chars.next().expect("unexpected end of input");
                    let c = self.chars.next().expect("unexpected end of input").1;
                    if c == '\'' {
                        break Token::Char;
                    }
                    break Token::Err;
                }
                '/' => match self.chars.next()?.1 {
                    '/' => {
                        while let Some((_, c)) = self.chars.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                        start = self.curr_pos();
                        continue;
                    }
                    '*' => {
                        loop {
                            let c = self.chars.next().expect("unexpected end of input").1;
                            if c == '*' {
                                let c = self.chars.peek().expect("unexpected end of input").1;
                                if c == '/' {
                                    self.chars.next();
                                    break;
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
                    while let Some((_, c)) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            self.chars.next();
                            continue;
                        }
                        break;
                    }
                    break Token::Number;
                }
                '"' => {
                    loop {
                        let c = self.chars.next().expect("unexpected end of input").1;
                        if c == '"' {
                            break;
                        }
                    }
                    break Token::String;
                }
                '%' => match self.chars.next()?.1 {
                    '%' => {
                        self.percent_percent_count += 1;
                        if self.percent_percent_count >= 2 {
                            while let Some(_) = self.chars.next() {}
                            break Token::Epilogue;
                        }
                        break Token::PercentPercent;
                    }
                    '{' => {
                        loop {
                            let c = self.chars.next().expect("unexpected end of input").1;
                            if c == '%' {
                                let c = self.chars.peek().expect("unexpected end of input").1;
                                if c == '}' {
                                    self.chars.next();
                                    break;
                                }
                            }
                        }
                        break Token::Prologue;
                    }
                    'a'..='z' | 'A'..='Z' => {
                        while let Some((_, c)) = self.chars.peek() {
                            if c.is_ascii_alphanumeric() || *c == '_' || *c == '-' {
                                self.chars.next();
                                continue;
                            }
                            break;
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
                    loop {
                        let c = self.chars.next().expect("unexpected end of input").1;
                        match c {
                            '{' => depth += 1,
                            '}' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    break Token::Code;
                }
                'a'..='z' | 'A'..='Z' => {
                    while let Some((_, c)) = self.chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                self.chars.next();
                                continue;
                            }
                            _ => break,
                        }
                    }
                    break Token::Ident;
                }
                '<' => {
                    break loop {
                        let c = self.chars.next().expect("unexpected end of input").1;
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => continue,
                            '>' => break Token::Type,
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
