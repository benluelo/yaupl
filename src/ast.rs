pub trait Location {
    type Row: Usize;
    type Column: Usize;

    fn row(&self) -> Self::Row;

    fn col(&self) -> Self::Column;

    fn set_row(&self, val: usize);

    fn set_col(&self, val: usize);
}

pub trait Usize {}

impl Usize for usize {}
