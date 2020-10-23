use crate::{
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{comma, tuple_close, tuple_open},
};

use super::{yaupl_type, Type};

pub(crate) fn tuple(i: &str, ptr: Pointer) -> Result<(&str, Pointer, Type), ParseError> {
    let mut found_types = vec![];
    let (i, ptr, _bracket) = tuple_open(i, ptr)?;
    let (mut i, mut ptr) = (i, ptr);
    loop {
        if let Ok(found_type) = yaupl_type(i, ptr) {
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
