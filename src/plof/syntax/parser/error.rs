use super::lexer::TokenPosition;

use std::fmt;

#[derive(Debug)]
pub enum ParserErrorValue {
    Constant(String),
}

#[derive(Debug)]
pub struct ParserError {
    value:    ParserErrorValue,
    position: Option<TokenPosition>,
}

impl ParserError {
    pub fn new(value: &str) -> ParserError {
        ParserError {
            value: ParserErrorValue::Constant(value.to_owned()),
            position: None,
        }
    }

    pub fn new_pos(position: TokenPosition, value: &str) -> ParserError {
        ParserError {
            value: ParserErrorValue::Constant(value.to_owned()),
            position: Some(position),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            ParserErrorValue::Constant(ref s) => match self.position {
                Some(p) => write!(f, "{}: {}", p, s),
                None    => write!(f, "{}", s),
            }
        }
    }
}
