use std::{cmp::Ordering, fmt::Debug};

pub trait Token: Debug {
    fn token(&self) -> &str;
}

impl<'a> PartialEq for Box<dyn Token + 'a> {
    fn eq(&self, other: &Box<dyn Token + 'a>) -> bool {
        self.token().eq(other.token())
    }
}

impl<'a> Eq for Box<dyn Token + 'a> {}

impl<'a> PartialOrd for Box<dyn Token + 'a> {
    fn partial_cmp(&self, other: &Box<dyn Token + 'a>) -> Option<Ordering> {
        self.token().partial_cmp(other.token())
    }
}

impl<'a> Ord for Box<dyn Token + 'a> {
    fn cmp(&self, other: &Box<dyn Token + 'a>) -> Ordering {
        self.token().cmp(other.token())
    }
}
