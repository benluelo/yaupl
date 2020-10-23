use self::span::Span;

pub(crate) mod span;
#[derive(Debug, Eq, Hash, PartialEq, std::cmp::Ord, std::cmp::PartialOrd)]
pub(crate) struct AstNode<T: ?Sized> {
    pub(crate) location: Span,
    pub(crate) body: T,
}

#[allow(dead_code)]
impl<T> AstNode<T> {
    pub(crate) fn new(location: Span, body: T) -> Self {
        Self { location, body }
    }

    pub(crate) fn boxed(self) -> AstNode<Box<T>> {
        AstNode {
            location: self.location,
            body: Box::new(self.body),
        }
    }
}