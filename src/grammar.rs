// rule1; rule2; rule3;
#[derive(Debug)]
pub struct Grammar {
    pub directives: Vec<Directive>,
    pub rules: Vec<Rule>,
    pub prologues: Vec<String>,
    pub epilogue: String,
}

#[derive(Debug)]
pub enum Directive {
    Code,
    Define,
    Header,
    Verbose,
    InitialAction,
    Printer,
    Destructor,

    // %pure-parser
    // ------------
    // Tells yacc (or Bison) to generate a pure parser, which means that the parser
    // will not use a global variable like yylval to store the semantic
    // value of tokens. Instead, the parser will use a local instance of YYSTYPE
    // to store the semantic value of tokens, passing it as a parameter to functions.
    // YYSTYPE is the type used for semantic values, and it can be defined
    // using the %union directive if multiple types are needed.
    PureParser,
    // %expect number
    // ------------
    // tells yacc the expected number of	shift/reduce conflicts. That
    // makes it only report the number if it differs.
    Expect {
        number: i32,
    },
    // %name-prefix="prefix"
    // ------------
    // Rename the external symbols used in the parser so  that
    // they start with prefix instead of yy.  The precise list
    // of symbols renamed is yyparse, yylex, yyerror,  yylval,
    // yychar, and yydebug.
    NamePrefix {
        prefix: String,
    },
    // %locations
    // ------------
    // Enables locations tracking
    Locations,
    // %parse-param { params }
    // ------------
    // Adds a parameter to the yyparse function signature.
    ParseParam {
        params: String,
    },
    // %lex-param { params }
    // Adds a parameter to the yylex function signature.
    LexProgram {
        params: String,
    },
    // %union { code }
    Union {
        code: String,
    },
    // %type <type> identifiers
    Type {
        type_name: String,
        rule_names: Vec<String>,
    },
    // %token [<token>] identifiers
    Token {
        token_name: Option<String>,
        rule_names: Vec<String>,
    },
    // %left identifiers
    Left {
        rule_names: Vec<String>,
    },
    // %right identifiers
    Right {
        rule_names: Vec<String>,
    },
    // %nonassoc identifiers
    NonAssoc {
        rule_names: Vec<String>,
    },
}

// foo: bar baz { ... } | qux { ...};
#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug)]
pub struct Alternative {
    pub elements: Vec<String>,
    pub precedence: Option<String>,
    pub action: Option<String>,
}
