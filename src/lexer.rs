use crate::token::Spanned;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    percent_percent_count: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input[self.pos..].chars();
        let mut start = self.pos;
        let token = loop {
            let c = chars.next()?;
            match c {
                '\'' => {
                    self.advance();
                    chars.next().expect("unexpected end of input");
                    self.advance();
                    let c = chars.next().expect("unexpected end of input");
                    self.advance();
                    if c == '\'' {
                        break Token::Char;
                    }
                    break Token::Err;
                }
                // '<char>'
                '/' => {
                    self.advance();
                    match chars.next()? {
                        '/' => {
                            self.advance();
                            while let Some(c) = chars.next() {
                                if c == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                            start = self.pos;
                            continue;
                        }
                        '*' => {
                            self.advance();
                            loop {
                                let c = chars.next().expect("unexpected end of input");
                                self.advance();
                                if c == '*' {
                                    let c = chars.next().expect("unexpected end of input");
                                    if c == '/' {
                                        self.advance();
                                        break;
                                    } else {
                                        chars = self.input[self.pos..].chars();
                                    }
                                }
                            }
                            start = self.pos;
                            continue;
                        }
                        _ => break Token::Err,
                    }
                }
                '\n' | ' ' | '\t' => {
                    self.advance();
                    start = self.pos;
                    continue;
                }
                '=' => {
                    self.advance();
                    break Token::Equal;
                }
                '0'..='9' => {
                    self.advance();
                    while let Some(c) = chars.next() {
                        if c.is_ascii_digit() {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                    break Token::Number;
                }
                '"' => {
                    self.advance();
                    while let Some(c) = chars.next() {
                        self.advance();
                        if c == '"' {
                            break;
                        }
                    }
                    break Token::String;
                }
                '%' => {
                    self.advance();
                    match chars.next()? {
                        '%' => {
                            self.advance();
                            self.percent_percent_count += 1;
                            if self.percent_percent_count >= 2 {
                                while let Some(_) = chars.next() {
                                    self.advance();
                                }
                                break Token::Epilogue;
                            }
                            break Token::PercentPercent;
                        }
                        '{' => {
                            self.advance();
                            loop {
                                let c = chars.next()?;
                                self.advance();
                                if c == '%' {
                                    let c = chars.next()?;
                                    self.advance();
                                    if c == '}' {
                                        break;
                                    }
                                }
                            }
                            break Token::Prologue;
                        }
                        'a'..='z' | 'A'..='Z' => {
                            self.advance();
                            while let Some(c) = chars.next() {
                                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                                    self.advance();
                                    continue;
                                }
                                break;
                            }
                            break Token::Directive;
                        }
                        _ => {
                            self.advance();
                            break Token::Err;
                        }
                    }
                }
                '|' => {
                    self.advance();
                    break Token::Bar;
                }
                ':' => {
                    self.advance();
                    break Token::Colon;
                }
                ';' => {
                    self.advance();
                    break Token::SemiColon;
                }
                // {...}
                '{' => {
                    self.advance();
                    let mut depth = 1;
                    loop {
                        let c = chars.next().expect("unexpected end of input");
                        self.advance();
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
                    self.advance();
                    while let Some(c) = chars.next() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                self.advance();
                                continue;
                            }
                            _ => break,
                        }
                    }
                    break Token::Ident;
                }
                '<' => {
                    self.advance();
                    break loop {
                        let c = chars.next().expect("unexpected end of input");
                        self.advance();
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => continue,
                            '>' => break Token::Type,
                            _ => break Token::Err,
                        }
                    };
                }
                _ => {
                    self.advance();
                    Token::Err
                }
            };
        };
        Some(Spanned::new(token, start..self.pos))
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            pos: 0,
            percent_percent_count: 0,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
