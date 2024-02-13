use std::cmp::Ordering;

use crate::{Avltriee, AvltrieeAllocator, AvltrieeSearch, AvltrieeUpdate};

impl<T: Ord + Clone, A: AvltrieeAllocator<T>> AvltrieeSearch<T, T, A> for Avltriee<T, T, A> {
    fn cmp(left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }

    fn invert<'a>(&'a self, value: &'a T) -> &T {
        value
    }
}

impl<T: Ord + Clone, A: AvltrieeAllocator<T>> AvltrieeUpdate<T, T, A> for Avltriee<T, T, A> {
    fn convert_on_insert_unique(&mut self, input: &T) -> T {
        input.clone()
    }
}

impl<T, I: ?Sized, A> AsRef<Avltriee<T, I, A>> for Avltriee<T, I, A> {
    fn as_ref(&self) -> &Avltriee<T, I, A> {
        self
    }
}
impl<T, I: ?Sized, A> AsMut<Avltriee<T, I, A>> for Avltriee<T, I, A> {
    fn as_mut(&mut self) -> &mut Avltriee<T, I, A> {
        self
    }
}
