use std::num::NonZeroU32;

use crate::Avltriee;

impl<T> Avltriee<T> {
    fn delete_same(&mut self, delete_row: NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked(delete_row) };

        let delete_node_same = delete_node.same.unwrap();
        let delete_node_parent = delete_node.parent;
        let delete_node_height = delete_node.height;
        let delete_node_left = delete_node.left;
        let delete_node_right = delete_node.right;

        let new_node = unsafe { self.get_unchecked_mut(delete_node_same) };

        new_node.parent = delete_node_parent;
        new_node.height = delete_node_height;
        new_node.left = delete_node_left;
        new_node.right = delete_node_right;

        if let Some(left) = delete_node_left {
            unsafe { self.get_unchecked_mut(left) }.parent = Some(delete_node_same);
        }
        if let Some(right) = delete_node_right {
            unsafe { self.get_unchecked_mut(right) }.parent = Some(delete_node_same);
        }
    }

    fn delete_intermediate(&mut self, delete_row: NonZeroU32) -> (NonZeroU32, NonZeroU32) {
        let delete_node = unsafe { self.get_unchecked(delete_row) };

        let delete_node_left = delete_node.left.unwrap();
        let delete_node_right = delete_node.right.unwrap();
        let delete_node_parent = delete_node.parent;
        let delete_node_height = delete_node.height;

        let left_max = unsafe { NonZeroU32::new_unchecked(self.max(delete_node_left.get())) };

        unsafe { self.get_unchecked_mut(left_max) }.right = Some(delete_node_right);
        unsafe { self.get_unchecked_mut(delete_node_right) }.parent = Some(left_max);

        if delete_node_left == left_max {
            unsafe { self.get_unchecked_mut(left_max) }.parent = delete_node_parent;
            self.reset_height(left_max);
            (left_max, left_max)
        } else {
            let left_max_node = unsafe { self.get_unchecked_mut(left_max) };

            let left_max_parent = left_max_node.parent.unwrap();
            let left_max_left = left_max_node.left;

            left_max_node.height = delete_node_height;

            left_max_node.left = Some(delete_node_left);
            unsafe { self.get_unchecked_mut(delete_node_left) }.parent = Some(left_max);

            unsafe { self.get_unchecked_mut(left_max_parent) }.right = left_max_left;
            if let Some(right) = left_max_left {
                unsafe { self.get_unchecked_mut(right) }.parent = Some(left_max_parent);
            }

            (left_max, left_max_parent)
        }
    }

    /// Delete the specified row.
    pub fn delete(&mut self, row: NonZeroU32) {
        if let Some(node) = self.get(row) {
            let row_parent = node.parent;
            let same = node.same;
            if let Some(row_parent) = row_parent {
                let parent_same = unsafe { self.get_unchecked_mut(row_parent) }.same;
                if parent_same == Some(row) {
                    unsafe { self.get_unchecked_mut(row_parent) }.same = same;
                    if same.is_some() {
                        self.delete_same(row);
                    }
                } else if let Some(same) = same {
                    unsafe { self.get_unchecked_mut(row_parent) }.changeling(row, Some(same));
                    self.delete_same(row);
                } else {
                    let left = unsafe { self.get_unchecked(row) }.left;
                    let right = unsafe { self.get_unchecked(row) }.right;
                    if left.is_none() {
                        unsafe { self.get_unchecked_mut(row_parent) }.changeling(row, right);
                        if let Some(right) = right {
                            unsafe { self.get_unchecked_mut(right) }.parent = Some(row_parent);
                        }
                        self.balance(row_parent);
                    } else if right.is_none() {
                        unsafe { self.get_unchecked_mut(row_parent) }.changeling(row, left);
                        if let Some(left) = left {
                            unsafe { self.get_unchecked_mut(left) }.parent = Some(row_parent);
                        }
                        self.balance(row_parent);
                    } else {
                        let (new_row, balance_row) = self.delete_intermediate(row);
                        unsafe { self.get_unchecked_mut(row_parent) }
                            .changeling(row, Some(new_row));
                        let delete_row_height = unsafe { self.get_unchecked(row) }.height;
                        let node = unsafe { self.get_unchecked_mut(new_row) };
                        node.height = delete_row_height;
                        node.parent = Some(row_parent);
                        self.reset_height(balance_row);
                        self.balance(balance_row);
                    }
                }
            } else {
                if let Some(same) = same {
                    self.set_root(same.get());
                    self.delete_same(row);
                } else {
                    let left = node.left;
                    let right = node.right;
                    if left.is_none() {
                        let right = right.unwrap();
                        self.set_root(right.get());
                        unsafe { self.get_unchecked_mut(right) }.parent = None;
                    } else if right.is_none() {
                        let left = left.unwrap();
                        self.set_root(left.get());
                        unsafe { self.get_unchecked_mut(left) }.parent = None;
                    } else {
                        let (new_row, balance_row) = self.delete_intermediate(row);
                        self.set_root(new_row.get());
                        unsafe { self.get_unchecked_mut(new_row) }.parent = None;
                        self.reset_height(balance_row);
                        self.balance(balance_row);
                    }
                }
            }
            unsafe { self.get_unchecked_mut(row) }.height = None;

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
