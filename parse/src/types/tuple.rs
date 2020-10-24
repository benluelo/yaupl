use crate::{
    combinators::csv,
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{tuple_close, tuple_open},
};

use super::{yaupl_type, Type};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tuple(pub Vec<Type>);

/// REFACTOR
/// TODO: generalize this into a comma-seperated value function
pub(crate) fn tuple(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, _bracket) = tuple_open(i, ptr)?;
    let (i, ptr, csv) = csv(i, ptr, &yaupl_type)?;
    let (i, ptr, _bracket) = tuple_close(i, ptr)?;

    Ok((i, ptr, Type::Tuple(Tuple(csv))))
}
