use std::collections::BTreeMap;

use crate::{
    parse_error::ParseError, pointer::Pointer, tokens::group, types::complex::Complex,
    types::function::Function, types::tuple::Tuple, Identifier,
};

use self::primitive::PrimitiveType;

// use super::{Identifier, parse_error::ParseError, pointer::Pointer, tokens::group};

pub(crate) mod complex;
pub(crate) mod function;
pub(crate) mod primitive;
pub(crate) mod tuple;

pub(crate) fn yaupl_type(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, mut res) = primitive::primitive(i, ptr)
        .or(function::function(i, ptr))
        .or(tuple::tuple(i, ptr))
        .or(complex::complex(i, ptr))?;

    let (mut i, mut ptr) = (i, ptr);
    loop {
        if let Ok((new_i, new_ptr, _group_sigil)) = group(i, ptr) {
            i = new_i;
            ptr = new_ptr;
            res = Type::Group(Group(Box::new(res)));
        } else {
            return Ok((i, ptr, res));
        };
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Group(Box<Type>);

// REFACTOR: make the enum variants tuple structs containing their respective data
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
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
    Tuple(Tuple),
    /// Used for an unsized collection of something of the same type.
    /// ### Examples
    /// ```yaupl
    /// num@@
    /// ```
    Group(Group),
    /// Synonymous to a struct, a key: value pair of types.
    /// ### Examples
    /// ```yaupl
    /// [a: str, b: bln@, c: [num, bln]=>___]
    /// ```
    Complex(Complex),
    /// A function that takes the types of the left side of the arrow and returns the right side.
    /// ### Examples
    /// ```yaupl
    /// [num, bln]=>___
    /// ```
    Function(Function),
}
