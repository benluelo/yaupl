use crate::{parse_error::ParseError, pointer::Pointer, tokens::arrow_right_thick};

use super::{tuple::tuple, yaupl_type, Type};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Function {
    parameters: Vec<Type>,
    return_type: Box<Type>,
}

/// REFACTOR
/// TODO: can't rely on the tuple function anymore since the tuples have a different syntax (`[| |]` vs `[ ]`)
pub(crate) fn function(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let (i, ptr, parameters) = tuple(i, ptr)?;

    let parameters = match parameters {
        Type::Tuple(t) => t,
        _ => unreachable!(),
    };

    let (i, ptr, _arrow) = arrow_right_thick(i, ptr)?;

    let (i, ptr, return_type) = yaupl_type(i, ptr)?;

    Ok((
        i,
        ptr,
        Type::Function(Function {
            parameters: parameters.0,
            return_type: Box::new(return_type),
        }),
    ))
}
