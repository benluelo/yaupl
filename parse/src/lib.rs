#![feature(or_patterns, trait_alias, associated_type_bounds)]
use std::fmt::Debug;

use crate::types::{yaupl_type, Type};

use self::{
    combinators::{not, one_of},
    parse_error::ParseError,
    pointer::Pointer,
    tokens::token::Token,
    tokens::*,
    whitespace::whitespace,
};

pub fn parse(i: &str) -> Result<(&str, Pointer, Type), ParseError> {
    yaupl_type(i, Pointer::new(0, 0))
}
// use crate::ast::defs::{types::*, *};

pub(crate) mod combinators;
pub(crate) mod expression;
pub(crate) mod parse_error;
pub(crate) mod pointer;
pub(crate) mod tokens;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod whitespace;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Token for Digit {
    fn token(&self) -> &str {
        match self {
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9",
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Self::Zero
    }
}

impl From<char> for Digit {
    fn from(ch: char) -> Self {
        match ch {
            '0' => Digit::Zero,
            '1' => Digit::One,
            '2' => Digit::Two,
            '3' => Digit::Three,
            '4' => Digit::Four,
            '5' => Digit::Five,
            '6' => Digit::Six,
            '7' => Digit::Seven,
            '8' => Digit::Eight,
            '9' => Digit::Nine,
            _ => panic!("Invalid digit: {}", ch),
        }
    }
}
// impl Digit {
//     pub(crate) fn from_char(ch: char) -> Digit {
//     }
// }
pub(crate) fn digit(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Digit), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    if let Some(ch) = i.chars().next() {
        if ch.is_ascii_digit() {
            Ok((&i[1..], ptr.add_col(1), ch.into()))
        } else {
            Err(ParseError::ExpectedDigit)
        }
    } else {
        Err(ParseError::None)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Identifier(String);
impl Token for Identifier {
    fn token(&self) -> &str {
        &*self.0
    }
}
fn ident(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Identifier), ParseError> {
    let (i, ptr) = whitespace(i, ptr);
    let mut ch_inds = i.char_indices();
    let mut prev_ind = 0;
    let end_location = loop {
        match ch_inds.next() {
            Some((ind, 'a'..='z' | 'A'..='Z' | '_')) => {
                prev_ind = ind;
                continue;
            }
            Some((ind, _)) => break ind,
            None => {
                if prev_ind > 0 {
                    break prev_ind;
                } else {
                    return Err(ParseError::None);
                }
            }
        };
    };

    not((&i[end_location..]).clone(), ptr.clone(), &infinity)?;
    // this one's not technically possible, but it's here for posterity
    not(
        (&i[end_location..]).clone(),
        ptr.clone(),
        &negative_infinity,
    )?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_true)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_false)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_export)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_return)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_with)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_as)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_str)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_num)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_bln)?;
    not((&i[..end_location]).clone(), ptr.clone(), &keyword_emp)?;

    Ok((
        &i[end_location..],
        ptr.add_col(end_location),
        Identifier(i[..end_location].into()),
    ))
}
