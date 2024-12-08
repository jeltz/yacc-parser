use crate::grammar::Alternative;
use crate::grammar::Directive;
use crate::grammar::Grammar;
use crate::grammar::Rule;

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for directive in &self.directives {
            writeln!(f, "{}", directive)?;
        }
        writeln!(f, "%%")?;
        for rule in &self.rules {
            writeln!(f, "{}", rule)?;
        }
        writeln!(f, "%%")?;
        for line in self.epilogue.lines() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Directive::PureParser => writeln!(f, "%pure-parser"),
            Directive::Expect { number } => writeln!(f, "%expect {}", number),
            Directive::NamePrefix { prefix } => writeln!(f, "%name-prefix=\"{}\"", prefix),
            Directive::Locations => writeln!(f, "%locations"),
            Directive::ParseParam { params } => writeln!(f, "%parse-param {{ {} }}", params),
            Directive::LexProgram { params } => writeln!(f, "%lex-param {{ {} }}", params),
            Directive::Union { code } => writeln!(f, "%union {{ {} }}", code),
            Directive::Type {
                type_name,
                rule_names,
            } => {
                write!(f, "%type {}", type_name)?;
                for rule_name in rule_names {
                    write!(f, " {}", rule_name)?;
                }
                writeln!(f)
            }
            Directive::Token {
                token_name,
                rule_names,
            } => {
                write!(f, "%token")?;
                if let Some(token) = token_name {
                    write!(f, " {}", token)?;
                }
                for rule_name in rule_names {
                    write!(f, " {}", rule_name)?;
                }
                writeln!(f)
            }
            Directive::Left { rule_names } => {
                write!(f, "%left")?;
                for rule_name in rule_names {
                    write!(f, " {}", rule_name)?;
                }
                writeln!(f)
            }
            Directive::Right { rule_names } => {
                write!(f, "%right")?;
                for rule_name in rule_names {
                    write!(f, " {}", rule_name)?;
                }
                writeln!(f)
            }
            Directive::NonAssoc { rule_names } => {
                write!(f, "%nonassoc")?;
                for rule_name in rule_names {
                    write!(f, " {}", rule_name)?;
                }
                writeln!(f)
            }
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for alternative in &self.alternatives {
            writeln!(f, "    |{}", alternative)?;
        }
        writeln!(f, ";")
    }
}

impl std::fmt::Display for Alternative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.elements.iter() {
            write!(f, " {}", element)?;
        }
        if let Some(precedence) = &self.precedence {
            write!(f, " %prec {}", precedence)?;
        }
        if let Some(action) = &self.action {
            write!(f, " {}", action)?;
        }
        Ok(())
    }
}
