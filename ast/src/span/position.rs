#[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd, Copy, Clone)]
pub(crate) struct Position {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl From<(usize, usize)> for Position {
    fn from(val: (usize, usize)) -> Self {
        Position {
            row: val.0,
            col: val.1,
        }
    }
}

#[allow(dead_code)]
impl Position {
    pub(crate) fn new(val: (usize, usize)) -> Self {
        Position {
            row: val.0,
            col: val.1,
        }
    }
}
