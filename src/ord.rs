use std::cmp::Ordering;

use crate::{search, Avltriee, AvltrieeAllocator, Found};

pub trait AvltrieeOrd<T, I: ?Sized, A: AvltrieeAllocator<T>>: AsRef<Avltriee<T, I, A>> {
    fn cmp(&self, left: &T, right: &I) -> Ordering;

    /// Finds the edge of a node from the specified value.
    fn search(&self, value: &I) -> Found
    where
        Self: Sized,
    {
        search::edge(self, value)
    }
}
