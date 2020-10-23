pub(crate) mod position;
use self::position::Position;

#[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd, Copy, Clone)]
pub(crate) struct Span {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl From<((usize, usize), (usize, usize))> for Span {
    fn from(tuple: ((usize, usize), (usize, usize))) -> Self {
        Span {
            start: tuple.0.into(),
            end: tuple.1.into(),
        }
    }
}
