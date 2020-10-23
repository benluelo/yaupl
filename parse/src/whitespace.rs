use super::pointer::Pointer;

pub(crate) fn whitespace(i: &str, ptr: Pointer) -> (&str, Pointer) {
    let mut ch_inds = i.char_indices();
    let mut prev_ind = 0;
    let mut rows = 0;
    let mut cols = 0;
    let end_location = loop {
        match ch_inds.next() {
            Some((ind, ch @ ('\n' | '\r' | '\t' | ' '))) => {
                cols += 1;
                if ch == '\n' || ch == '\r' {
                    rows += 1;
                    cols = 0;
                }
                prev_ind = ind;
                continue;
            }
            Some((ind, _)) => break ind,
            None => break prev_ind,
        };
    };
    (&i[end_location..], ptr.add_col(cols).add_row(rows))
}

#[cfg(test)]
mod test_white_space {
    use crate::pointer::Pointer;

    use super::*;

    #[test]
    fn test_white_space() {
        assert_eq!(
            whitespace(" \n        hello", Pointer { row: 0, col: 0 }),
            ("hello", Pointer { row: 1, col: 8 })
        );
    }
}
