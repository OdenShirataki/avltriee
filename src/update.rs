mod balance;
mod delete;

use std::{cmp::Ordering, num::NonZeroU32};

use async_trait::async_trait;

use super::{Avltriee, AvltrieeNode, Found};

impl<T> AsRef<Avltriee<T>> for Avltriee<T> {
    fn as_ref(&self) -> &Avltriee<T> {
        self
    }
}
impl<T> AsMut<Avltriee<T>> for Avltriee<T> {
    fn as_mut(&mut self) -> &mut Avltriee<T> {
        self
    }
}

#[async_trait(?Send)]
pub trait AvltrieeHolder<T, I>: AsRef<Avltriee<T>> + AsMut<Avltriee<T>> {
    fn cmp(&self, left: &T, right: &I) -> Ordering;
    fn search(&self, input: &I) -> Found;
    fn convert_value(&mut self, input: I) -> T;
    async fn delete_before_update(&mut self, row: NonZeroU32);
}

#[async_trait(?Send)]
impl<T: Ord> AvltrieeHolder<T, T> for Avltriee<T> {
    fn cmp(&self, left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }

    fn search(&self, input: &T) -> Found {
        self.search(|v| v.cmp(input))
    }

    fn convert_value(&mut self, input: T) -> T {
        input
    }

    async fn delete_before_update(&mut self, row: NonZeroU32) {
        self.delete(row);
    }
}

impl<T> Avltriee<T> {
    /// Creates a new row and assigns a value to it.
    pub async fn insert(&mut self, value: T) -> NonZeroU32
    where
        T: Ord + Clone + Default,
    {
        let row = unsafe { NonZeroU32::new_unchecked(self.rows_count() + 1) };
        self.update(row, value).await;
        row
    }

    /// Updates the value in the specified row.
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    pub async fn update(&mut self, row: NonZeroU32, value: T)
    where
        T: Ord + Clone + Default,
    {
        Self::update_with_holder(self, row, value).await;
    }

    /// Updates the value of the specified row via trait [AvltrieeHolder].
    /// If you specify a row that does not exist, space will be automatically allocated. If you specify a row that is too large, memory may be allocated unnecessarily.
    pub async fn update_with_holder<I>(
        holder: &mut dyn AvltrieeHolder<T, I>,
        row: NonZeroU32,
        input: I,
    ) where
        T: Clone + Default,
    {
        if let Some(node) = holder.as_ref().get(row) {
            if holder.cmp(node, &input) == Ordering::Equal {
                return; //update value eq exists value
            }
            holder.delete_before_update(row).await;
        }

        let found = holder.search(&input);
        if found.ord == Ordering::Equal && found.row != 0 {
            let same_row = unsafe { NonZeroU32::new_unchecked(found.row) };

            holder.as_mut().allocate(row);

            let same_node = unsafe { holder.as_mut().get_unchecked_mut(same_row) };
            let same_left = NonZeroU32::new(same_node.left);
            let same_right = NonZeroU32::new(same_node.right);
            let same_parent = same_node.parent;

            *unsafe { holder.as_mut().get_unchecked_mut(row) } =
                same_node.same_clone(same_row, row);

            holder.as_mut().replace_child(same_parent, same_row, row);

            if let Some(left) = same_left {
                unsafe { holder.as_mut().get_unchecked_mut(left) }.parent = row.get();
            }
            if let Some(right) = same_right {
                unsafe { holder.as_mut().get_unchecked_mut(right) }.parent = row.get();
            }
        } else {
            let value = holder.convert_value(input);
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

        *self.get_unchecked_mut(row) = AvltrieeNode::new(row.get(), found.row, value);
        if found.row == 0 {
            self.set_root(row.get());
        } else {
            let p = self.get_unchecked_mut(NonZeroU32::new_unchecked(found.row));
            if found.ord == Ordering::Greater {
                p.left = row.get();
            } else {
                p.right = row.get();
            }
            self.balance(row);
        }
    }

    fn reset_height(&mut self, row: NonZeroU32) -> u8 {
        let node = unsafe { self.get_unchecked(row) };
        let left_height = if node.left != 0 {
            unsafe { self.get_unchecked(NonZeroU32::new_unchecked(node.left)) }.height
        } else {
            0
        };
        let right_height = if node.right != 0 {
            unsafe { self.get_unchecked(NonZeroU32::new_unchecked(node.right)) }.height
        } else {
            0
        };
        let height = std::cmp::max(left_height, right_height) + 1;
        unsafe { self.get_unchecked_mut(row) }.height = height;
        height
    }

    fn replace_child(&mut self, parent: u32, current_child: NonZeroU32, new_child: NonZeroU32) {
        if parent == 0 {
            self.set_root(new_child.get());
        } else {
            unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(parent)) }
                .changeling(current_child, new_child);
        }
    }
}
