//! Hand-rolled parser for the predicate DSL.
//!
//! Grammar:
//!     expr     := or_expr
//!     or_expr  := and_expr ('or' and_expr)*
//!     and_expr := primary ('and' primary)*
//!     primary  := '(' expr ')' | comparison
//!     comparison := IDENT (('==' | '!=' | 'in' | 'not in') literal)
//!     literal  := STRING | INT | '[' literal (',' literal)* ']'

use crate::notification_contract::predicates::{Literal, Predicate};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("predicate parse error: {0}")]
    Msg(String),
}

pub fn parse_predicate(source: &str) -> Result<Predicate, ParseError> {
    let tokens = tokenize(source)?;
    let mut parser = Parser { tokens: &tokens, pos: 0 };
    let expr = parser.parse_or()?;
    if parser.pos != tokens.len() {
        return Err(ParseError::Msg(format!(
            "trailing tokens at position {}",
            parser.pos
        )));
    }
    Ok(expr)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Str(String),
    Int(i64),
    EqEq, NotEq,
    KwAnd, KwOr, KwIn, KwNot,
    LParen, RParen,
    LBracket, RBracket,
    Comma,
}

fn tokenize(source: &str) -> Result<Vec<Token>, ParseError> {
    let mut out = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => { chars.next(); }
            '(' => { chars.next(); out.push(Token::LParen); }
            ')' => { chars.next(); out.push(Token::RParen); }
            '[' => { chars.next(); out.push(Token::LBracket); }
            ']' => { chars.next(); out.push(Token::RBracket); }
            ',' => { chars.next(); out.push(Token::Comma); }
            '=' => {
                chars.next();
                if chars.next() != Some('=') {
                    return Err(ParseError::Msg("expected '=='".into()));
                }
                out.push(Token::EqEq);
            }
            '!' => {
                chars.next();
                if chars.next() != Some('=') {
                    return Err(ParseError::Msg("expected '!='".into()));
                }
                out.push(Token::NotEq);
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('"') => break,
                        Some(c) => s.push(c),
                        None => return Err(ParseError::Msg("unterminated string literal".into())),
                    }
                }
                out.push(Token::Str(s));
            }
            d if d.is_ascii_digit() => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() { s.push(c); chars.next(); } else { break; }
                }
                out.push(Token::Int(s.parse().map_err(|e| ParseError::Msg(format!("bad int: {e}")))?));
            }
            a if a.is_ascii_alphabetic() || a == '_' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' { s.push(c); chars.next(); } else { break; }
                }
                let tok = match s.as_str() {
                    "and" => Token::KwAnd,
                    "or" => Token::KwOr,
                    "in" => Token::KwIn,
                    "not" => Token::KwNot,
                    _ => Token::Ident(s),
                };
                out.push(tok);
            }
            other => return Err(ParseError::Msg(format!("unexpected character: {other:?}"))),
        }
    }
    Ok(out)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn bump(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    fn parse_or(&mut self) -> Result<Predicate, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Token::KwOr)) {
            self.bump();
            let right = self.parse_and()?;
            left = Predicate::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Predicate, ParseError> {
        let mut left = self.parse_primary()?;
        while matches!(self.peek(), Some(Token::KwAnd)) {
            self.bump();
            let right = self.parse_primary()?;
            left = Predicate::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Predicate, ParseError> {
        if matches!(self.peek(), Some(Token::LParen)) {
            self.bump();
            let inner = self.parse_or()?;
            if !matches!(self.bump(), Some(Token::RParen)) {
                return Err(ParseError::Msg("expected ')'".into()));
            }
            return Ok(inner);
        }
        let field = match self.bump() {
            Some(Token::Ident(s)) => s.clone(),
            other => return Err(ParseError::Msg(format!("expected identifier, got {other:?}"))),
        };
        match self.bump() {
            Some(Token::EqEq) => {
                let lit = self.parse_literal_single()?;
                Ok(Predicate::Equals { field, value: lit })
            }
            Some(Token::NotEq) => {
                let lit = self.parse_literal_single()?;
                Ok(Predicate::NotEquals { field, value: lit })
            }
            Some(Token::KwIn) => {
                let lits = self.parse_literal_list()?;
                Ok(Predicate::In { field, values: lits })
            }
            Some(Token::KwNot) => {
                if !matches!(self.bump(), Some(Token::KwIn)) {
                    return Err(ParseError::Msg("expected 'not in'".into()));
                }
                let lits = self.parse_literal_list()?;
                Ok(Predicate::NotIn { field, values: lits })
            }
            other => Err(ParseError::Msg(format!("expected comparator, got {other:?}"))),
        }
    }

    fn parse_literal_single(&mut self) -> Result<Literal, ParseError> {
        match self.bump() {
            Some(Token::Str(s)) => Ok(Literal::Str(s.clone())),
            Some(Token::Int(i)) => Ok(Literal::Int(*i)),
            other => Err(ParseError::Msg(format!("expected literal, got {other:?}"))),
        }
    }

    fn parse_literal_list(&mut self) -> Result<Vec<Literal>, ParseError> {
        if !matches!(self.bump(), Some(Token::LBracket)) {
            return Err(ParseError::Msg("expected '['".into()));
        }
        let mut out = Vec::new();
        if matches!(self.peek(), Some(Token::RBracket)) {
            self.bump();
            return Ok(out);
        }
        loop {
            out.push(self.parse_literal_single()?);
            match self.bump() {
                Some(Token::Comma) => continue,
                Some(Token::RBracket) => break,
                other => return Err(ParseError::Msg(format!("expected ',' or ']', got {other:?}"))),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_equals() {
        let p = parse_predicate(r#"status == "FUNDS_RECEIVED""#).unwrap();
        assert_eq!(p, Predicate::Equals {
            field: "status".into(),
            value: Literal::Str("FUNDS_RECEIVED".into()),
        });
    }

    #[test]
    fn parses_in() {
        let p = parse_predicate(r#"status in ["A", "B"]"#).unwrap();
        assert_eq!(p, Predicate::In {
            field: "status".into(),
            values: vec![Literal::Str("A".into()), Literal::Str("B".into())],
        });
    }

    #[test]
    fn parses_not_in() {
        let p = parse_predicate(r#"status not in ["PENDING"]"#).unwrap();
        assert_eq!(p, Predicate::NotIn {
            field: "status".into(),
            values: vec![Literal::Str("PENDING".into())],
        });
    }

    #[test]
    fn parses_and_or() {
        let p = parse_predicate(r#"a == "X" or b == "Y" and c == "Z""#).unwrap();
        // `and` binds tighter, so: Or(a==X, And(b==Y, c==Z))
        assert!(matches!(p, Predicate::Or(_, _)));
    }

    #[test]
    fn parses_parens() {
        let p = parse_predicate(r#"(a == "X" or b == "Y") and c == "Z""#).unwrap();
        assert!(matches!(p, Predicate::And(_, _)));
    }

    #[test]
    fn rejects_trailing_garbage() {
        assert!(parse_predicate(r#"a == "X" garbage"#).is_err());
    }

    #[test]
    fn rejects_incomplete() {
        assert!(parse_predicate(r#"a =="#).is_err());
    }
}
