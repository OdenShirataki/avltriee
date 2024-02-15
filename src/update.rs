mod balance;
mod delete;

use std::{cmp::Ordering, num::NonZeroU32};

use crate::{search::Edge, AvltrieeAllocator, AvltrieeSearch};

use super::{Avltriee, AvltrieeNode};

pub trait AvltrieeUpdate<T, I: ?Sized, A: AvltrieeAllocator<T>>:
    AsMut<Avltriee<T, I, A>> + AvltrieeSearch<T, I, A>
{
    fn convert_on_insert_unique(&mut self, input: &I) -> T;
    fn on_delete(&mut self, _row: NonZeroU32) {}

    /// Creates a new row and assigns a value to it.
    fn insert(&mut self, value: &I) -> NonZeroU32
    where
        T: Clone,
    {
        let row = unsafe { NonZeroU32::new_unchecked(self.as_ref().rows_count() + 1) };
        self.update(row, value);
        row
    }

    /// Updates the value in the specified row.
    fn update(&mut self, row: NonZeroU32, value: &I)
    where
        T: Clone,
    {
        if let Some(node_value) = self.value(row) {
            if Self::cmp(node_value, value) == Ordering::Equal {
                return; //update value eq exists value
            }
            self.delete(row);
        }

        let edge = self.edge(value);
        if let (Some(same_row), Ordering::Equal) = edge {
            let triee = self.as_mut();

            triee.allocate(row);

            let same_node = unsafe { triee.node_unchecked_mut(same_row) };
            let same_left = same_node.left;
            let same_right = same_node.right;
            let same_parent = same_node.parent;

            *unsafe { triee.node_unchecked_mut(row) } = same_node.same_clone(same_row, row);

            triee.replace_child(same_parent, same_row, Some(row));

            if let Some(left) = same_left {
                unsafe { triee.node_unchecked_mut(left) }.parent = Some(row);
            }
            if let Some(right) = same_right {
                unsafe { triee.node_unchecked_mut(right) }.parent = Some(row);
            }
        } else {
            let value = self.convert_on_insert_unique(value);
            unsafe { self.as_mut().insert_unique_unchecked(row, value, edge) };
        }
    }

    /// Delete the specified row.
    fn delete(&mut self, row: NonZeroU32) {
        self.on_delete(row);
        self.as_mut().delete_inner(row);
    }
}

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
    /// Insert a unique value.
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    /// # Safety
    /// value ​​must be unique.
    pub unsafe fn insert_unique_unchecked(&mut self, row: NonZeroU32, value: T, edge: Edge) {
        self.allocate(row);

        *self.node_unchecked_mut(row) = AvltrieeNode::new(edge.0, value);
        if let Some(found_row) = edge.0 {
            let p = self.node_unchecked_mut(found_row);
            if edge.1 == Ordering::Greater {
                p.left = Some(row);
            } else {
                p.right = Some(row);
            }
            self.balance(row);
        } else {
            self.set_root(Some(row));
        }
    }

    fn reset_height(&mut self, row: NonZeroU32) {
        let node = unsafe { self.node_unchecked(row) };

        let left_height = node.left.map_or(
            0,
            |left: NonZeroU32| unsafe { self.node_unchecked(left) }.height,
        );

        let right_height = node
            .right
            .map_or(0, |right| unsafe { self.node_unchecked(right) }.height);

        unsafe { self.node_unchecked_mut(row) }.height =
            std::cmp::max(left_height, right_height) + 1;
    }

    fn replace_child(
        &mut self,
        parent: Option<NonZeroU32>,
        current_child: NonZeroU32,
        new_child: Option<NonZeroU32>,
    ) {
        if let Some(parent) = parent {
            unsafe { self.node_unchecked_mut(parent) }.changeling(current_child, new_child);
        } else {
            self.set_root(new_child);
        }
    }
}
