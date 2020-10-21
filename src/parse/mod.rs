use std::{collections::BTreeMap, fmt::Debug};

use pointer::Pointer;
use tokens::*;

use self::{combinators::one_of, tokens::token::Token, whitespace::whitespace};

// use crate::ast::defs::{types::*, *};

pub(crate) mod combinators;
pub(crate) mod pointer;
#[allow(dead_code)]
pub(crate) mod tokens;
pub(crate) mod whitespace;

#[derive(Debug)]
#[allow(dead_code)]
#[non_exhaustive]
pub(crate) enum ParseError {
    Expected(Box<dyn Token + 'static>),
    ExpectedOneOf(Vec<Box<dyn Token + 'static>>),
    None,
    OneOf,
    OneOrMore,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Identifier(String);
impl Token for Identifier {
    fn token(&self) -> &str {
        &*self.0
    }
}
// TODO: make sure the identifier found isn't a keyword
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
    Ok((
        &i[end_location..],
        ptr.add_row(end_location),
        Identifier(i[..end_location].into()),
    ))
}

#[cfg(test)]
mod test_types {
    use super::*;

    #[test]
    fn test_primitive() {
        assert_eq!(
            primitive("    str,  not_str", Pointer { row: 0, col: 0 }).unwrap(),
            (
                ",  not_str",
                Pointer { row: 0, col: 7 },
                Type::Primitive(PrimitiveType::Str),
            )
        );
    }
}

pub(crate) fn primitive<'a>(
    i: &'a str,
    ptr: Pointer,
) -> Result<(&'a str, Pointer, Type), ParseError> {
    one_of(
        i,
        ptr,
        &[&keyword_str, &keyword_num, &keyword_bln, &keyword_emp],
    )
    .map(|res| (res.0, res.1, Type::Primitive(res.2)))
}

pub(crate) fn tuple(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let mut found_types = vec![];
    let (i, ptr, _bracket) = tuple_open(i, ptr)?;
    let (mut i, mut ptr) = (i, ptr);
    loop {
        if let Ok(found_type) = r#type(i, ptr) {
            i = found_type.0;
            ptr = found_type.1;
            found_types.push(found_type.2);
            if let Ok(comma) = comma(i, ptr) {
                i = comma.0;
                ptr = comma.1;
            }
        } else {
            break;
        }
    }
    let (i, ptr, _bracket) = tuple_close(i, ptr)?;

    Ok((i, ptr, Type::Tuple(found_types)))
}

pub(crate) fn r#type(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, mut res) = primitive(i, ptr)
        .or(function(i, ptr))
        .or(tuple(i, ptr))
        .or(complex(i, ptr))?;

    let (mut i, mut ptr) = (i, ptr);
    loop {
        if let Ok((new_i, new_ptr, _group_sigil)) = group(i, ptr) {
            i = new_i;
            ptr = new_ptr;
            res = Type::Group(Box::new(res));
        } else {
            return Ok((i, ptr, res));
        };
    }
}

pub(crate) fn function(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, parameters) = tuple(i, ptr)?;

    let parameters = match parameters {
        Type::Tuple(t) => t,
        _ => unreachable!(),
    };

    let (i, ptr, _arrow) = arrow_right_thick(i, ptr)?;

    let (i, ptr, return_type) = r#type(i, ptr)?;

    Ok((
        i,
        ptr,
        Type::Function {
            parameters,
            return_type: Box::new(return_type),
        },
    ))
}

pub(crate) fn complex(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let mut map = BTreeMap::new();

    let (i, ptr, _bracket) = tesla_open(i, ptr)?;

    let (mut i, mut ptr) = (i, ptr);
    loop {
        if let Ok(kvp) = key_value_pair(i, ptr) {
            map.insert(kvp.2.key.0, kvp.2.value.0);
            if let Ok(comma) = comma(kvp.0, kvp.1) {
                i = comma.0;
                ptr = comma.1;
            } else {
                i = kvp.0;
                ptr = kvp.1;
                break;
            }
        } else {
            break;
        }
    }

    let (i, ptr, _bracket) = tesla_close(i, ptr)?;

    Ok((i, ptr, Type::Complex(map)))
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum PrimitiveType {
    Str,
    Bln,
    Num,
    Emp,
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum Type {
    /// The basic types.
    /// ### Examples
    /// ```yaupl
    /// str
    /// ```
    Primitive(PrimitiveType),
    /// Used for a collection of loosely-related types. The empty tuple is currently ***forbidden***
    /// ### Examples
    /// ```yaupl
    /// [str, num, [str, bln]=>num]
    /// ```
    Tuple(Vec<Type>),
    /// Used for an unsized collection of something of the same type.
    /// ### Examples
    /// ```yaupl
    /// num@@
    /// ```
    Group(Box<Type>),
    /// Synonymous to a struct, a key: value pair of types.
    /// ### Examples
    /// ```yaupl
    /// [a: str, b: bln@, c: [num, bln]=>___]
    /// ```
    Complex(BTreeMap<Identifier, Type>),
    /// A function that takes the types of the left side of the arrow and returns the right side.
    /// ### Examples
    /// ```yaupl
    /// [num, bln]=>___
    /// ```
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

pub(crate) fn key_value_pair(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeyValuePair<Identifier, Type>), ParseError> {
    let (i, ptr, identifier) = ident(i, ptr)?;
    let ident_ptr = ptr.clone();
    let (i, ptr, _colon) = colon(i, ptr)?;
    let (i, ptr, found_type) = r#type(i, ptr)?;
    let type_ptr = ptr.clone();
    Ok((
        i,
        ptr,
        KeyValuePair {
            key: (identifier, ident_ptr),
            value: (found_type, type_ptr),
        },
    ))

    // todo!();
}

#[derive(Debug)]
pub(crate) struct KeyValuePair<K, V> {
    key: (K, Pointer),
    value: (V, Pointer),
}

// impl From<ParseError> for ParseError {
//     fn from(tok: ParseError) -> Self {
//         tok.as_dyn()
//     }
// }
