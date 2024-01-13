use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    pub(crate) fn balance(&mut self, row: NonZeroU32) {
        let mut t: &AvltrieeNode<T> = unsafe { self.get_unchecked(row) };
        while t.parent != 0 {
            let u_row = unsafe { NonZeroU32::new_unchecked(t.parent) };
            let u = unsafe { self.get_unchecked(u_row) };

            let height_before_balance = u.height;

            let left_row = unsafe { NonZeroU32::new_unchecked(u.left) };
            let right_row = unsafe { NonZeroU32::new_unchecked(u.right) };

            let left = unsafe { self.get_unchecked(left_row) };
            let right = unsafe { self.get_unchecked(right_row) };

            let (t_row, new_height) = match left.height as isize - right.height as isize {
                2 => {
                    if self.height(left.left) < self.height(left.right) {
                        self.rotate_left(left_row);
                    }
                    (right_row, self.rotate_right(u_row))
                }
                -2 => {
                    if self.height(right.left) > self.height(right.right) {
                        self.rotate_right(right_row);
                    }
                    (left_row, self.rotate_left(u_row))
                }
                _ => (u_row, self.reset_height(u_row)),
            };
            if height_before_balance == new_height {
                break;
            }
            t = unsafe { self.get_unchecked(t_row) };
        }
    }

    fn height(&self, row: u32) -> u8 {
        if let Some(row) = NonZeroU32::new(row) {
            unsafe { self.get_unchecked(row) }.height
        } else {
            0
        }
    }

    fn rotate_common(&mut self, row: NonZeroU32, child_row: NonZeroU32) -> u8 {
        let node_parent = unsafe { self.get_unchecked(row) }.parent;

        self.replace_child(node_parent, row, child_row);

        let new_height = self.reset_height(row);
        self.reset_height(child_row);

        unsafe { self.get_unchecked_mut(child_row) }.parent = node_parent;
        unsafe { self.get_unchecked_mut(row) }.parent = child_row.get();

        new_height
    }

    fn rotate_left(&mut self, row: NonZeroU32) -> u8 {
        let right_row = unsafe { NonZeroU32::new_unchecked(self.get_unchecked(row).right) };
        let right_left = unsafe { self.get_unchecked(right_row) }.left;

        unsafe { self.get_unchecked_mut(row) }.right = right_left;
        if let Some(right) = NonZeroU32::new(right_left) {
            unsafe { self.get_unchecked_mut(right) }.parent = row.get();
        }
        unsafe { self.get_unchecked_mut(right_row) }.left = row.get();

        self.rotate_common(row, right_row)
    }

    fn rotate_right(&mut self, row: NonZeroU32) -> u8 {
        let left_row = unsafe { NonZeroU32::new_unchecked(self.get_unchecked(row).left) };
        let left_right = unsafe { self.get_unchecked(left_row) }.right;

        unsafe { self.get_unchecked_mut(row) }.left = left_right;
        if let Some(left) = NonZeroU32::new(left_right) {
            unsafe { self.get_unchecked_mut(left) }.parent = row.get();
        }
        unsafe { self.get_unchecked_mut(left_row) }.right = row.get();

        self.rotate_common(row, left_row)
    }
}
