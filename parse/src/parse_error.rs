use crate::{pointer::Pointer, tokens::token::Token};

#[derive(Debug)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum ParseError {
    Expected(Box<dyn Token + 'static>),
    ExpectedOneOf(Vec<Box<dyn Token + 'static>>),
    UnexpectedKeyword(Box<dyn Token + 'static>),
    None,
    OneOf,
    OneOrMoe,
    ExpectedDigit,
    UnterminatedStringLiteral,
}

impl<T: Token + 'static> From<(&str, Pointer, T)> for ParseError {
    fn from(f: (&str, Pointer, T)) -> Self {
        ParseError::UnexpectedKeyword(Box::new(f.2) as Box<dyn Token>)
    }
}
