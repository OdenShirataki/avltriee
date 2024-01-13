use std::num::NonZeroU32;

use crate::Avltriee;

impl<T> Avltriee<T> {
    fn delete_same(&mut self, delete_row: NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked(delete_row) };

        let delete_node_same = unsafe { NonZeroU32::new_unchecked(delete_node.same) };
        let delete_node_parent = delete_node.parent;
        let delete_node_height = delete_node.height;
        let delete_node_left = delete_node.left;
        let delete_node_right = delete_node.right;

        let new_node = unsafe { self.get_unchecked_mut(delete_node_same) };

        new_node.parent = delete_node_parent;
        new_node.height = delete_node_height;
        new_node.left = delete_node_left;
        new_node.right = delete_node_right;

        if let Some(left) = NonZeroU32::new(delete_node_left) {
            unsafe { self.get_unchecked_mut(left) }.parent = delete_node_same.get();
        }
        if let Some(right) = NonZeroU32::new(delete_node_right) {
            unsafe { self.get_unchecked_mut(right) }.parent = delete_node_same.get();
        }
    }

    fn delete_intermediate(&mut self, delete_row: NonZeroU32) -> (NonZeroU32, NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked(delete_row) };

        let delete_node_left = unsafe { NonZeroU32::new_unchecked(delete_node.left) };
        let delete_node_right = unsafe { NonZeroU32::new_unchecked(delete_node.right) };
        let delete_node_parent = delete_node.parent;
        let delete_node_height = delete_node.height;

        let left_max = unsafe { NonZeroU32::new_unchecked(self.max(delete_node_left.get())) };

        unsafe { self.get_unchecked_mut(left_max) }.right = delete_node_right.get();
        unsafe { self.get_unchecked_mut(delete_node_right).parent = left_max.get() };

        if delete_node_left == left_max {
            unsafe { self.get_unchecked_mut(left_max) }.parent = delete_node_parent;
            self.reset_height(left_max);
            (left_max, left_max)
        } else {
            let left_max_node = unsafe { self.get_unchecked_mut(left_max) };

            let left_max_parent = unsafe { NonZeroU32::new_unchecked(left_max_node.parent) };
            let left_max_left = left_max_node.left;

            left_max_node.height = delete_node_height;

            left_max_node.left = delete_node_left.get();
            unsafe { self.get_unchecked_mut(delete_node_left).parent = left_max.get() };

            unsafe { self.get_unchecked_mut(left_max_parent) }.right = left_max_left;
            if let Some(right) = NonZeroU32::new(left_max_left) {
                unsafe { self.get_unchecked_mut(right) }.parent = left_max_parent.get();
            }

            (left_max, left_max_parent)
        }
    }

    /// Delete the specified row.
    pub fn delete(&mut self, row: NonZeroU32) {
        if self.get(row).is_some() {
            let node = unsafe { self.get_unchecked(row) };
            let row_parent = node.parent;
            let same = node.same;
            if row_parent == 0 {
                if same != 0 {
                    self.set_root(same);
                    self.delete_same(row);
                } else {
                    let left = node.left;
                    let right = node.right;
                    if left == 0 {
                        self.set_root(right);
                        if let Some(right) = NonZeroU32::new(right) {
                            unsafe { self.get_unchecked_mut(right) }.parent = 0;
                        }
                    } else if right == 0 {
                        self.set_root(left);
                        unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(left)) }.parent =
                            0;
                    } else {
                        let (new_row, balance_row) = self.delete_intermediate(row);
                        self.set_root(new_row.get());
                        unsafe { self.get_unchecked_mut(new_row) }.parent = 0;
                        self.reset_height(balance_row);
                        self.balance(balance_row);
                    }
                }
            } else {
                let row_parent = unsafe { NonZeroU32::new_unchecked(row_parent) };
                let parent_same = unsafe { self.get_unchecked_mut(row_parent) }.same;
                if parent_same == row.get() {
                    unsafe { self.get_unchecked_mut(row_parent) }.same = same;
                    if same != 0 {
                        self.delete_same(row);
                    }
                } else if same != 0 {
                    unsafe { self.get_unchecked_mut(row_parent) }
                        .changeling(row, unsafe { NonZeroU32::new_unchecked(same) });
                    self.delete_same(row);
                } else {
                    let left = unsafe { self.get_unchecked(row) }.left;
                    let right = unsafe { self.get_unchecked(row) }.right;
                    if left == 0 {
                        unsafe { self.get_unchecked_mut(row_parent) }
                            .changeling(row, unsafe { NonZeroU32::new_unchecked(right) });
                        if let Some(right) = NonZeroU32::new(right) {
                            unsafe { self.get_unchecked_mut(right) }.parent = row_parent.get();
                        }
                        self.balance(row_parent);
                    } else if right == 0 {
                        unsafe { self.get_unchecked_mut(row_parent) }
                            .changeling(row, unsafe { NonZeroU32::new_unchecked(left) });
                        unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(left)) }.parent =
                            row_parent.get();
                        self.balance(row_parent);
                    } else {
                        let (new_row, balance_row) = self.delete_intermediate(row);
                        unsafe { self.get_unchecked_mut(row_parent) }.changeling(row, new_row);
                        let delete_row_height = unsafe { self.get_unchecked(row) }.height;
                        let node = unsafe { self.get_unchecked_mut(new_row) };
                        node.height = delete_row_height;
                        node.parent = row_parent.get();
                        self.reset_height(balance_row);
                        self.balance(balance_row);
                    }
                }
            }
            unsafe { self.get_unchecked_mut(row) }.height = 0;

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
