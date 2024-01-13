use std::num::{NonZeroU32, NonZeroU8};

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    pub(crate) fn balance(&mut self, row: NonZeroU32) {
        let mut t: &AvltrieeNode<T> = unsafe { self.get_unchecked(row) };
        while let Some(u_row) = t.parent {
            let u = unsafe { self.get_unchecked(u_row) };

            let height_before_balance = u.height.unwrap();

            let (t_row, new_height) =
                match self.height(u.left) as isize - self.height(u.right) as isize {
                    2 => {
                        let right_row = u.right;
                        let left_row = u.left.unwrap();
                        let left = unsafe { self.get_unchecked(left_row) };
                        if self.height(left.left) < self.height(left.right) {
                            self.rotate_left(left_row);
                        }
                        (right_row, self.rotate_right(u_row))
                    }
                    -2 => {
                        let left_row = u.left;
                        let right_row = u.right.unwrap();
                        let right = unsafe { self.get_unchecked(right_row) };
                        if self.height(right.left) > self.height(right.right) {
                            self.rotate_right(right_row);
                        }
                        (left_row, self.rotate_left(u_row))
                    }
                    _ => (Some(u_row), self.reset_height(u_row)),
                };
            if height_before_balance == new_height {
                break;
            }
            if let Some(t_row) = t_row {
                t = unsafe { self.get_unchecked(t_row) };
            } else {
                break;
            }
        }
    }

    fn height(&self, row: Option<NonZeroU32>) -> u8 {
        if let Some(row) = row {
            unsafe { self.get_unchecked(row) }.height.unwrap().get()
        } else {
            0
        }
    }

    fn rotate_common(&mut self, row: NonZeroU32, child_row: NonZeroU32) -> NonZeroU8 {
        let node_parent = unsafe { self.get_unchecked(row) }.parent;

        self.replace_child(node_parent, row, Some(child_row));

        let new_height = self.reset_height(row);
        self.reset_height(child_row);

        unsafe { self.get_unchecked_mut(child_row) }.parent = node_parent;
        unsafe { self.get_unchecked_mut(row) }.parent = Some(child_row);

        new_height
    }

    fn rotate_left(&mut self, row: NonZeroU32) -> NonZeroU8 {
        let right_row = unsafe { self.get_unchecked(row) }.right.unwrap();
        let right_left = unsafe { self.get_unchecked(right_row) }.left;

        unsafe { self.get_unchecked_mut(row) }.right = right_left;
        if let Some(right) = right_left {
            unsafe { self.get_unchecked_mut(right) }.parent = Some(row);
        }
        unsafe { self.get_unchecked_mut(right_row) }.left = Some(row);

        self.rotate_common(row, right_row)
    }

    fn rotate_right(&mut self, row: NonZeroU32) -> NonZeroU8 {
        let left_row = unsafe { self.get_unchecked(row) }.left.unwrap();
        let left_right = unsafe { self.get_unchecked(left_row) }.right;

        unsafe { self.get_unchecked_mut(row) }.left = left_right;
        if let Some(left) = left_right {
            unsafe { self.get_unchecked_mut(left) }.parent = Some(row);
        }
        unsafe { self.get_unchecked_mut(left_row) }.right = Some(row);

        self.rotate_common(row, left_row)
    }
}
