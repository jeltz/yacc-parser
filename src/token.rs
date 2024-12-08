#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Equal,          // =
    PercentPercent, // %%
    GreaterThan,    // >
    LessThan,       // <
    Bar,            // |
    Colon,          // :
    SemiColon,      // ;
    Code,           // { ... }
    Prologue,       // %{ ... %}
    Ident,          // [a-zA-Z_][a-zA-Z0-9_]*|'.'
    String,         // '...'
    Eof,            // End of file
    Directive,      // %ident ...
    Char,           // 'a'
    Number,         // 123
    Type,
    Err,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub data: T,
    pub span: std::ops::Range<usize>,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: std::ops::Range<usize>) -> Self {
        Spanned { data, span }
    }
}
