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
            let same = found.row;
            let t = holder.as_mut();

            t.allocate(row);

            let same_node = unsafe { t.get_unchecked_mut(NonZeroU32::new_unchecked(same)) };
            let node = unsafe { t.get_unchecked_mut(row) };

            *node = same_node.clone();

            t.change_row(node, unsafe { NonZeroU32::new_unchecked(same) }, row);

            let row_prim = row.get();
            same_node.parent = row_prim;
            node.same = same;
            if let Some(left) = NonZeroU32::new(node.left) {
                t.set_parent(left, row_prim);
            }
            if let Some(right) = NonZeroU32::new(node.right) {
                t.set_parent(right, row_prim);
            }
            same_node.left = 0;
            same_node.right = 0;
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

    fn calc_height(&mut self, row: NonZeroU32) {
        let node = unsafe { self.get_unchecked_mut(row) };
        self.calc_height_node(node);
    }

    fn calc_height_node(&self, node: &mut AvltrieeNode<T>) {
        node.height = unsafe {
            std::cmp::max(
                if node.left != 0 {
                    self.get_unchecked(NonZeroU32::new_unchecked(node.left))
                        .height
                } else {
                    0
                },
                if node.right != 0 {
                    self.get_unchecked(NonZeroU32::new_unchecked(node.right))
                        .height
                } else {
                    0
                },
            )
        } + 1;
    }

    fn join_intermediate(
        parent: &mut AvltrieeNode<T>,
        target_row: NonZeroU32,
        child_row: NonZeroU32,
    ) {
        let target_row = target_row.get();
        if parent.right == target_row {
            parent.right = child_row.get();
        } else if parent.left == target_row {
            parent.left = child_row.get();
        }
    }

    fn change_row(
        &mut self,
        node: &mut AvltrieeNode<T>,
        target_row: NonZeroU32,
        child_row: NonZeroU32,
    ) {
        if node.parent == 0 {
            self.set_root(child_row.get());
        } else {
            Self::join_intermediate(
                unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(node.parent)) },
                target_row,
                child_row,
            );
        }
    }

    fn set_parent(&mut self, row: NonZeroU32, parent: u32) {
        unsafe { self.get_unchecked_mut(row) }.parent = parent;
    }
}
