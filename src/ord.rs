use std::cmp::Ordering;

use crate::{Avltriee, AvltrieeAllocator};

pub trait AvltrieeOrd<T, I, A>: AsRef<Avltriee<T, I, A>>
where
    I: ?Sized,
{
    fn cmp(&self, left: &T, right: &I) -> Ordering;
}

impl<T: Ord, A: AvltrieeAllocator<T>> AvltrieeOrd<T, T, A> for Avltriee<T, T, A> {
    fn cmp(&self, left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }
}
