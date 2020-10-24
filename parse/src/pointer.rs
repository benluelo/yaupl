#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pointer {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

/// A pointer to somewhere in the parsed code.
impl Pointer {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Adds a row to the pointer. Rows are only encountered on newlines.
    pub fn add_row(self, row: usize) -> Self {
        Pointer::new(self.row + row, self.col)
    }

    /// Adds a column to the pointer.
    #[allow(dead_code)]
    pub fn add_col(self, col: usize) -> Self {
        Pointer::new(self.row, self.col + col)
    }
}

impl From<(usize, usize)> for Pointer {
    fn from(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<Pointer> for (usize, usize) {
    fn from(ptr: Pointer) -> Self {
        (ptr.row, ptr.col)
    }
}
