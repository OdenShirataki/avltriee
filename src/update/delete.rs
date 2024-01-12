use std::num::NonZeroU32;

use crate::Avltriee;

impl<T> Avltriee<T> {
    fn delete_same(&mut self, delete_row: NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked_mut(delete_row) };
        let new_node =
            unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(delete_node.same)) };

        new_node.parent = delete_node.parent;
        new_node.height = delete_node.height;

        new_node.left = delete_node.left;
        if let Some(left) = NonZeroU32::new(new_node.left) {
            unsafe { self.get_unchecked_mut(left) }.parent = delete_node.same;
        }

        new_node.right = delete_node.right;
        if let Some(right) = NonZeroU32::new(new_node.right) {
            unsafe { self.get_unchecked_mut(right) }.parent = delete_node.same;
        }
    }

    fn delete_intermediate(&mut self, delete_row: NonZeroU32) -> (NonZeroU32, NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked_mut(delete_row) };

        let left_max_row = unsafe { NonZeroU32::new_unchecked(self.max(delete_node.left)) };
        let left_max = unsafe { self.get_unchecked_mut(left_max_row) };

        left_max.right = delete_node.right;
        unsafe {
            self.get_unchecked_mut(NonZeroU32::new_unchecked(left_max.right))
                .parent = left_max_row.get()
        };

        if delete_node.left == left_max_row.get() {
            left_max.parent = delete_node.parent;
            self.calc_height(left_max_row);
            let left_max_row = left_max_row;
            (left_max_row, left_max_row)
        } else {
            left_max.height = delete_node.height;

            let left_max_parent_row = unsafe { NonZeroU32::new_unchecked(left_max.parent) };
            let left_max_parent = unsafe { self.get_unchecked_mut(left_max_parent_row) };

            left_max_parent.right = left_max.left;
            if let Some(right) = NonZeroU32::new(left_max_parent.right) {
                unsafe { self.get_unchecked_mut(right) }.parent = left_max_parent_row.get();
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
                        let node = unsafe { self.get_unchecked_mut(new_row) };
                        node.parent = 0;
                        self.calc_height(balance_row);
                        self.balance(balance_row);
                    }
                }
            } else {
                let parent =
                    unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(row_parent)) };
                if parent.same == row.get() {
                    parent.same = same;
                    if same != 0 {
                        self.delete_same(row);
                    }
                } else if same != 0 {
                    parent.changeling(row, unsafe { NonZeroU32::new_unchecked(same) });
                    self.delete_same(row);
                } else {
                    let left = unsafe { self.get_unchecked(row) }.left;
                    let right = unsafe { self.get_unchecked(row) }.right;
                    if left == 0 {
                        parent.changeling(row, unsafe { NonZeroU32::new_unchecked(right) });
                        if let Some(right) = NonZeroU32::new(right) {
                            unsafe { self.get_unchecked_mut(right) }.parent = row_parent;
                        }
                        self.balance(unsafe { NonZeroU32::new_unchecked(row_parent) });
                    } else if right == 0 {
                        parent.changeling(row, unsafe { NonZeroU32::new_unchecked(left) });
                        unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(left)) }.parent =
                            row_parent;
                        self.balance(unsafe { NonZeroU32::new_unchecked(row_parent) });
                    } else {
                        let (new_row, balance_row) = self.delete_intermediate(row);
                        parent.changeling(row, new_row);
                        let node = unsafe { self.get_unchecked_mut(new_row) };
                        node.height = unsafe { self.get_unchecked(row) }.height;
                        node.parent = row_parent;
                        self.calc_height(balance_row);
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
