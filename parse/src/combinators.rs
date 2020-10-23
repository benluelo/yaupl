use super::{pointer::Pointer, ParseError};

// pub(crate) fn optionally<'a, T: Token>(
//     f: impl Fn(&'a str, Pointer) -> Res<T>,
// ) -> impl Fn(&'a str, Pointer) -> Option<Res<'a, T>> {
//     move |i: &'a str, ptr: Pointer| match f(i, ptr) {
//         Ok(res) => Some(Ok(res)),
//         Err(_) => None,
//     }
// }

pub(crate) fn not<'a, /* 'f: 'a, */ T>(
    i: &'a str,
    ptr: Pointer,
    f: &'a dyn Fn(&'a str, Pointer) -> Result<(&'a str, Pointer, T), ParseError>,
) -> Result<ParseError, (&'a str, Pointer, T)> {
    match f(i, ptr) {
        Ok(ok) => Err(ok),
        Err(err) => Ok(err),
    }
}

pub(crate) fn one_of<'a, /* 'f: 'a, */ T>(
    i: &'a str,
    ptr: Pointer,
    fns: &'a [&'a dyn Fn(&'a str, Pointer) -> Result<(&'a str, Pointer, T), ParseError>],
) -> Result<(&'a str, Pointer, T), ParseError> {
    for f in fns {
        match f(i, ptr) {
            res @ Ok(_) => return res,
            Err(_) => {
                continue;
            }
        }
    }
    Err(ParseError::None)
}

// pub(crate) fn one_or_more<'a, T: Default + Token + 'static>(
//     f: impl Fn(&'a str, Pointer) -> Res<T>,
// ) -> impl Fn(&'a str, Pointer) -> Res<'a, Vec<T>> {
//     move |i: &'a str, ptr: Pointer| loop {
//         let mut i = i.clone();
//         let mut ptr = ptr.clone();
//         let mut found = Box::new(vec![]);
//         match f(i, ptr) {
//             Ok((fi, fptr, res)) => {
//                 found.push(*res);
//                 i = fi;
//                 ptr = fptr;
//                 continue;
//             }
//             Err(err) => match found.len() {
//                 0 => return Err(ParseError::Expected(Box::new(vec![T::default()]))),
//                 _ => return Ok((i, ptr, found)),
//             },
//         }
//     }
// }

// pub(crate) fn zero_or_more<'a, T: Default + Token + 'static>(
//     f: impl Fn(&'a str, Pointer) -> Res<T>,
// ) -> impl Fn(&'a str, Pointer) -> Res<'a, Vec<T>> {
//     move |i: &'a str, ptr: Pointer| loop {
//         let mut i = i.clone();
//         let mut ptr = ptr.clone();
//         let mut found = vec![];
//         match f(i, ptr) {
//             Ok((fi, fptr, res)) => {
//                 found.push(*res);
//                 i = fi;
//                 ptr = fptr;
//                 continue;
//             }
//             Err(err) => return Ok((i, ptr, Box::new(found))),
//         }
//     }
// }
