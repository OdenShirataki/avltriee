mod balance;
mod delete;

use std::{cmp::Ordering, num::NonZeroU32};

use crate::{ord::AvltrieeOrd, AvltrieeAllocator};

use super::{Avltriee, AvltrieeNode, Found};

pub trait AvltrieeUpdate<T, I: ?Sized, A>: AsMut<Avltriee<T, I, A>> + AvltrieeOrd<T, I, A> {
    fn convert_value(&mut self, input: &I) -> T;
    fn delete_before_update(&mut self, row: NonZeroU32);
}

impl<T: Ord + Clone, A: AvltrieeAllocator<T>> AvltrieeUpdate<T, T, A> for Avltriee<T, T, A> {
    fn convert_value(&mut self, input: &T) -> T {
        input.clone()
    }

    fn delete_before_update(&mut self, row: NonZeroU32) {
        self.delete(row);
    }
}

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
    /// Creates a new row and assigns a value to it.
    pub fn insert(&mut self, value: &I) -> NonZeroU32
    where
        T: Ord + Clone + Default,
        Self: AvltrieeUpdate<T, I, A>,
    {
        let row = unsafe { NonZeroU32::new_unchecked(self.rows_count() + 1) };
        self.update(row, value);
        row
    }

    /// Updates the value in the specified row.
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    pub fn update(&mut self, row: NonZeroU32, value: &I)
    where
        T: Ord + Clone + Default,
        Self: AvltrieeUpdate<T, I, A>,
    {
        Self::update_with(self, row, value);
    }

    /// Updates the value of the specified row via trait [AvltrieeUpdate].
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    pub fn update_with<H: AvltrieeUpdate<T, I, A> + AvltrieeOrd<T, I, A>>(
        holder: &mut H,
        row: NonZeroU32,
        input: &I,
    ) where
        T: Clone + Default,
    {
        if let Some(node) = holder.as_ref().get(row) {
            if holder.cmp(node, &input) == Ordering::Equal {
                return; //update value eq exists value
            }
            holder.delete_before_update(row);
        }

        let found = holder.as_ref().search_edge(holder, &input);
        if found.ord == Ordering::Equal && found.row.is_some() {
            let same_row = found.row.unwrap();

            holder.as_mut().allocate(row);

            let same_node = unsafe { holder.as_mut().get_unchecked_mut(same_row) };
            let same_left = same_node.left;
            let same_right = same_node.right;
            let same_parent = same_node.parent;

            *unsafe { holder.as_mut().get_unchecked_mut(row) } =
                same_node.same_clone(same_row, row);

            holder
                .as_mut()
                .replace_child(same_parent, same_row, Some(row));

            if let Some(left) = same_left {
                unsafe { holder.as_mut().get_unchecked_mut(left) }.parent = Some(row);
            }
            if let Some(right) = same_right {
                unsafe { holder.as_mut().get_unchecked_mut(right) }.parent = Some(row);
            }
        } else {
            let value = holder.convert_value(&input);
            unsafe { holder.as_mut().insert_unique_unchecked(row, value, found) };
        }
    }

    /// Insert a unique value.
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    /// # Safety
    /// value ​​must be unique.
    pub unsafe fn insert_unique_unchecked(&mut self, row: NonZeroU32, value: T, found: Found)
    where
        T: Clone + Default,
    {
        self.allocate(row);

        *self.get_unchecked_mut(row) = AvltrieeNode::new(found.row, value);
        if let Some(found_row) = found.row {
            let p = self.get_unchecked_mut(found_row);
            if found.ord == Ordering::Greater {
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
        let node = unsafe { self.get_unchecked(row) };

        let left_height = node
            .left
            .map_or(0, |left| unsafe { self.get_unchecked(left) }.height);

        let right_height = node
            .right
            .map_or(0, |right| unsafe { self.get_unchecked(right) }.height);

        unsafe { self.get_unchecked_mut(row) }.height =
            std::cmp::max(left_height, right_height) + 1;
    }

    fn replace_child(
        &mut self,
        parent: Option<NonZeroU32>,
        current_child: NonZeroU32,
        new_child: Option<NonZeroU32>,
    ) {
        if let Some(parent) = parent {
            unsafe { self.get_unchecked_mut(parent) }.changeling(current_child, new_child);
        } else {
            self.set_root(new_child);
        }
    }
}
