use crate::grammar::Alternative;
use crate::grammar::Directive;
use crate::grammar::Grammar;
use crate::grammar::Rule;
use crate::lexer::Lexer;
use crate::token::Spanned;
use crate::token::Token;

pub struct Parser<'a> {
    input: &'a str,
    lexer: std::iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, lexer: Lexer<'a>) -> Self {
        Parser {
            input,
            lexer: lexer.peekable(),
        }
    }

    fn next(&mut self) -> Spanned<Token> {
        self.debug("next");
        self.lexer.next().unwrap()
    }

    fn peek(&mut self) -> &Spanned<Token> {
        self.debug("peek");
        self.lexer.peek().unwrap()
    }

    #[allow(dead_code)]
    fn debug(&mut self, s: &str) {
        let peek = self.lexer.peek().unwrap().clone();
        let source = self.input[peek.span.clone().start..]
            .chars()
            .take(200)
            .collect::<String>();
        println!(
            "Peek: [{s}] {:?} ({}) at {}",
            peek.data,
            self.text(peek.clone()),
            source
        );
    }

    fn text(&self, spanned: Spanned<Token>) -> &str {
        &self.input[spanned.span.clone()]
    }

    fn expect(&mut self, token: Token) -> Spanned<Token> {
        let spanned = self.next();
        if spanned.data != token {
            panic!(
                "Expected {:?}, found {:?} ({}) at byte {:?}",
                token,
                spanned.data,
                self.text(spanned.clone()),
                spanned.span.start
            )
        }
        spanned
    }

    fn parse_head(&mut self) -> (Vec<Directive>, Vec<String>) {
        let mut directives = Vec::new();
        let mut prologues = Vec::new();
        loop {
            match self.peek().data {
                Token::Directive => directives.push(self.parse_directive()),
                Token::Prologue => prologues.push(self.parse_prologue()),
                _ => break,
            }
        }
        (directives, prologues)
    }

    fn parse_directive(&mut self) -> Directive {
        let directive = self.expect(Token::Directive);
        match &self.input[directive.span.clone()] {
            "%pure-parser" => Directive::PureParser,
            "%expect" => {
                let number = self.expect(Token::Number);
                Directive::Expect {
                    number: self.text(number).parse().unwrap(),
                }
            }
            "%name-prefix" => {
                self.expect(Token::Equal);
                let prefix = self.expect(Token::String);
                Directive::NamePrefix {
                    prefix: self.text(prefix).to_string(),
                }
            }
            "%locations" => Directive::Locations,
            "%parse-param" => {
                let params = self.expect(Token::Code);
                Directive::ParseParam {
                    params: self.input[params.span.clone()].to_string(),
                }
            }
            "%lex-param" => {
                let program = self.expect(Token::Code);
                Directive::LexProgram {
                    params: self.input[program.span.clone()].to_string(),
                }
            }
            "%union" => {
                let code = self.expect(Token::Code);
                Directive::Union {
                    code: self.input[code.span.clone()].to_string(),
                }
            }
            "%type" => {
                let type_name = self.expect(Token::Type);
                let mut rule_names = Vec::new();
                loop {
                    if !matches!(self.peek().data, Token::Ident) {
                        break;
                    }
                    let rule_name = self.expect(Token::Ident);
                    rule_names.push(self.input[rule_name.span.clone()].to_string());
                }
                Directive::Type {
                    type_name: self.input[type_name.span.clone()].to_string(),
                    rule_names,
                }
            }
            "%token" => {
                let token_name = if self.peek().data == Token::Type {
                    let token_name = self.expect(Token::Type);
                    Some(self.input[token_name.span.clone()].to_string())
                } else {
                    None
                };
                let mut rule_names = Vec::new();
                while let Some(ident) = self.rule_name() {
                    rule_names.push(ident);
                }
                Directive::Token {
                    token_name,
                    rule_names,
                }
            }
            "%left" => {
                let mut rule_names = Vec::new();
                while let Some(ident) = self.rule_name() {
                    rule_names.push(ident);
                }
                Directive::Left { rule_names }
            }
            "%right" => {
                let mut rule_names = Vec::new();
                while let Some(ident) = self.rule_name() {
                    rule_names.push(ident);
                }
                Directive::Right { rule_names }
            }
            "%nonassoc" => {
                let mut rule_names = Vec::new();
                while let Some(ident) = self.rule_name() {
                    rule_names.push(ident);
                }
                Directive::NonAssoc { rule_names }
            }
            t => panic!("Unknown directive '{t}'"),
        }
    }

    fn parse_prologue(&mut self) -> String {
        let prologue = self.expect(Token::Prologue);
        self.input[prologue.span.start + 2..prologue.span.end - 1].to_string()
    }

    fn rule_name(&mut self) -> Option<String> {
        match self.peek().data {
            Token::Ident => {
                let ident = self.expect(Token::Ident);
                Some(self.input[ident.span.clone()].to_string())
            }
            Token::Char => {
                let char = self.expect(Token::Char);
                Some(self.input[char.span.clone()].to_string())
            }
            _ => None,
        }
    }

    fn parse_rule(&mut self) -> Rule {
        let name_token = self.expect(Token::Ident);
        let name = self.input[name_token.span.clone()].to_string();
        self.expect(Token::Colon);

        let mut alternatives = Vec::new();
        loop {
            let mut elements = Vec::new();
            loop {
                match self.peek().data {
                    Token::Ident => {
                        let element = self.expect(Token::Ident);
                        elements.push(self.input[element.span.clone()].to_string());
                    }
                    Token::Char => {
                        let char = self.expect(Token::Char);
                        elements.push(self.input[char.span.clone()].to_string());
                    }
                    _ => break,
                }
            }

            let precedence = if let Token::Directive = self.peek().data {
                let directive = &self.input[self.expect(Token::Directive).span.clone()];
                if directive != "%prec" {
                    panic!("Excepted %prec, got {}", directive);
                }
                let prec = self.expect(Token::Ident);
                Some(self.input[prec.span.clone()].to_string())
            } else {
                None
            };

            let action = if let Token::Code = self.peek().data {
                let code = self.expect(Token::Code);
                Some(self.input[code.span.clone()].to_string())
            } else {
                None
            };

            alternatives.push(Alternative {
                elements,
                precedence,
                action,
            });

            // Check if there are more alternatives
            match self.peek().data {
                Token::Bar => {
                    self.expect(Token::Bar);
                }
                Token::SemiColon => {
                    self.expect(Token::SemiColon);
                    break;
                }
                _ => panic!("Expected '|' or ';', found {:?}", self.peek().data),
            }
        }

        Rule { name, alternatives }
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        while let Token::Ident = self.peek().data {
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_epilogue(&mut self) -> String {
        let epilogue = self.expect(Token::Epilogue);
        //self.expect(Token::Eof); // TODO: Broken
        self.input[epilogue.span.start + 2..epilogue.span.end].to_string()
    }

    pub fn parse_grammar(&mut self) -> Grammar {
        let (directives, prologues) = self.parse_head();
        self.expect(Token::PercentPercent);
        let rules = self.parse_rules();
        let epilogue = self.parse_epilogue();

        Grammar {
            directives,
            rules,
            prologues,
            epilogue,
        }
    }
}
