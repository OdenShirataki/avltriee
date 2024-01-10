use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    fn delete_same(&mut self, delete_node: &AvltrieeNode<T>) {
        let new_node =
            unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(delete_node.same)) };

        new_node.parent = delete_node.parent;
        new_node.height = delete_node.height;

        new_node.left = delete_node.left;
        if let Some(left) = NonZeroU32::new(new_node.left) {
            self.set_parent(left, delete_node.same);
        }

        new_node.right = delete_node.right;
        if let Some(right) = NonZeroU32::new(new_node.right) {
            self.set_parent(right, delete_node.same);
        }
    }

    fn delete_intermediate(
        &mut self,
        delete_node: &mut AvltrieeNode<T>,
    ) -> (NonZeroU32, NonZeroU32) {
        let left_max_row = unsafe { NonZeroU32::new_unchecked(self.max(delete_node.left)) };
        let left_max = unsafe { self.get_unchecked_mut(left_max_row) };

        left_max.right = delete_node.right;
        unsafe {
            self.get_unchecked_mut(NonZeroU32::new_unchecked(left_max.right))
                .parent = left_max_row.get()
        };

        if delete_node.left == left_max_row.get() {
            left_max.parent = delete_node.parent;
            self.calc_height_node(left_max);
            let left_max_row = left_max_row;
            (left_max_row, left_max_row)
        } else {
            left_max.height = delete_node.height;

            let left_max_parent_row = unsafe { NonZeroU32::new_unchecked(left_max.parent) };
            let left_max_parent = unsafe { self.get_unchecked_mut(left_max_parent_row) };

            left_max_parent.right = left_max.left;
            if let Some(right) = NonZeroU32::new(left_max_parent.right) {
                self.set_parent(right, left_max_parent_row.get());
            }

            left_max.left = delete_node.left;
            unsafe {
                self.get_unchecked_mut(NonZeroU32::new_unchecked(left_max.left))
                    .parent = left_max_row.get()
            };

            (left_max_row, left_max_parent_row)
        }
    }

    /// Delete the specified row.
    pub fn delete(&mut self, row: NonZeroU32) {
        if let Some(delete_node) = self.get_mut(row) {
            let row_parent = delete_node.parent;
            if row_parent == 0 {
                if delete_node.same != 0 {
                    self.set_root(delete_node.same);
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    self.set_root(delete_node.right);
                    if let Some(right) = NonZeroU32::new(delete_node.right) {
                        self.set_parent(right, 0);
                    }
                } else if delete_node.right == 0 {
                    self.set_root(delete_node.left);
                    unsafe {
                        self.get_unchecked_mut(NonZeroU32::new_unchecked(delete_node.left))
                    }
                    .parent = 0;
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    self.set_root(new_row.get());
                    let node = unsafe { self.get_unchecked_mut(new_row) };
                    node.parent = 0;
                    self.calc_height(balance_row);
                    self.balance(balance_row);
                }
            } else {
                let mut parent =
                    unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(row_parent)) };
                if parent.same == row.get() {
                    parent.same = delete_node.same;
                    if delete_node.same != 0 {
                        self.delete_same(delete_node);
                    }
                } else if delete_node.same != 0 {
                    Self::join_intermediate(parent, row, unsafe {
                        NonZeroU32::new_unchecked(delete_node.same)
                    });
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    Self::join_intermediate(&mut parent, row, unsafe {
                        NonZeroU32::new_unchecked(delete_node.right)
                    });
                    if let Some(right) = NonZeroU32::new(delete_node.right) {
                        self.set_parent(right, row_parent);
                    }
                    self.balance(unsafe { NonZeroU32::new_unchecked(row_parent) });
                } else if delete_node.right == 0 {
                    Self::join_intermediate(parent, row, unsafe {
                        NonZeroU32::new_unchecked(delete_node.left)
                    });
                    unsafe {
                        self.get_unchecked_mut(NonZeroU32::new_unchecked(delete_node.left))
                    }
                    .parent = row_parent;
                    self.balance(unsafe { NonZeroU32::new_unchecked(row_parent) });
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    Self::join_intermediate(parent, row, new_row);
                    let node = unsafe { self.get_unchecked_mut(new_row) };
                    node.height = delete_node.height;
                    node.parent = row_parent;
                    self.calc_height(balance_row);
                    self.balance(balance_row);
                }
            }
            delete_node.height = 0;

            if row.get() == self.rows_count() {
                let mut current = row.get() - 1;
                if current > 0 {
                    while let None =
                        unsafe { self.allocator.get(NonZeroU32::new_unchecked(current)) }
                    {
                        current -= 1;
                        if current == 0 {
                            break;
                        }
                    }
                }
                self.set_rows_count(current);
            }
        }
    }
}
