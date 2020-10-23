use crate::{parse_error::ParseError, pointer::Pointer, tokens::arrow_right_thick};

use super::{tuple::tuple, yaupl_type, Type};

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
        Type::Function {
            parameters,
            return_type: Box::new(return_type),
        },
    ))
}
