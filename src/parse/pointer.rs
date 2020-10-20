#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Pointer {
    pub(crate) row: usize,
    pub(crate) col: usize,
}
impl From<(usize, usize)> for Pointer {
    fn from(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl Pointer {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn add_row(self, row: usize) -> Self {
        Pointer::new(self.row + row, self.col)
    }

    #[allow(dead_code)]
    pub fn add_col(self, col: usize) -> Self {
        Pointer::new(self.row, self.col + col)
    }
}
