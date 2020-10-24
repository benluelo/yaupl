use super::{
    ident, parse_error::ParseError, pointer::Pointer, tokens::colon, types::yaupl_type,
    types::Type, Identifier,
};

pub(crate) fn key_value_pair(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, KeyValuePair<Identifier, Type>), ParseError> {
    let (i, ptr, identifier) = ident(i, ptr)?;
    let ident_ptr = ptr.clone();
    let (i, ptr, _colon) = colon(i, ptr)?;
    let (i, ptr, found_type) = yaupl_type(i, ptr)?;
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
    pub(crate) key: (K, Pointer),
    pub(crate) value: (V, Pointer),
}

impl<K, V> KeyValuePair<K, V> {
    pub(crate) fn new(key: (K, Pointer), value: (V, Pointer)) -> Self {
        Self { key, value }
    }
}
