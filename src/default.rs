use std::{cmp::Ordering, num::NonZeroU32, ops::Deref};

use crate::{Avltriee, AvltrieeAllocator, AvltrieeNode, AvltrieeSearch, AvltrieeUpdate};

impl<T: Ord + Clone, A: AvltrieeAllocator<T>> AvltrieeSearch<T, T, A> for Avltriee<T, T, A> {
    fn cmp(left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }

    /// Returns the value of the specified row. Returns None if the row does not exist.
    fn value(&self, row: NonZeroU32) -> Option<&T> {
        self.as_ref().node(row).map(|v| v.deref())
    }

    /// Returns the value of the specified row.
    unsafe fn value_unchecked(&self, row: NonZeroU32) -> &T {
        self.as_ref().node_unchecked(row)
    }

    /// Returns node adn value of the specified row.
    unsafe fn node_value_unchecked(&self, row: NonZeroU32) -> (&AvltrieeNode<T>, &T) {
        let node = self.as_ref().node_unchecked(row);
        (node, node)
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
