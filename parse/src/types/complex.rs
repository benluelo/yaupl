use std::collections::BTreeMap;

use crate::{
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{comma, tesla_close, tesla_open},
    utils::key_value_pair,
};

use super::Type;

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
