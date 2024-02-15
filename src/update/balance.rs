use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeAllocator, AvltrieeNode};

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
    pub(crate) fn balance(&mut self, row: NonZeroU32) {
        let mut t: &AvltrieeNode<T> = unsafe { self.node_unchecked(row) };
        while let Some(mut u_row) = t.parent {
            let u = unsafe { self.node_unchecked(u_row) };

            let left_height = self.height(u.left);
            let right_height = self.height(u.right);

            match left_height as isize - right_height as isize {
                2 => {
                    let t_row = u.left.unwrap();
                    let left = unsafe { self.node_unchecked(t_row) };
                    if self.height(left.left) < self.height(left.right) {
                        self.rotate_left(t_row);
                    }
                    self.rotate_right(u_row);
                    u_row = t_row;
                }
                -2 => {
                    let t_row = u.right.unwrap();
                    let right = unsafe { self.node_unchecked(t_row) };
                    if self.height(right.left) > self.height(right.right) {
                        self.rotate_right(t_row);
                    }
                    self.rotate_left(u_row);
                    u_row = t_row;
                }
                _ => {
                    let new_height = std::cmp::max(left_height, right_height) + 1;
                    if u.height == new_height {
                        break;
                    }
                    unsafe { self.node_unchecked_mut(u_row) }.height = new_height;
                }
            };
            t = unsafe { self.node_unchecked(u_row) };
        }
    }

    fn height(&self, row: Option<NonZeroU32>) -> u8 {
        row.map_or(0, |row| unsafe { self.node_unchecked(row) }.height)
    }

    fn rotate_common(&mut self, row: NonZeroU32, child_row: NonZeroU32) {
        let node_parent = unsafe { self.node_unchecked(row) }.parent;

        self.replace_child(node_parent, row, Some(child_row));

        self.reset_height(row);
        self.reset_height(child_row);

        unsafe { self.node_unchecked_mut(child_row) }.parent = node_parent;
        unsafe { self.node_unchecked_mut(row) }.parent = Some(child_row);
    }

    fn rotate_left(&mut self, row: NonZeroU32) {
        let right_row = unsafe { self.node_unchecked(row) }.right.unwrap();
        let right_left = unsafe { self.node_unchecked(right_row) }.left;

        unsafe { self.node_unchecked_mut(row) }.right = right_left;
        if let Some(right) = right_left {
            unsafe { self.node_unchecked_mut(right) }.parent = Some(row);
        }
        unsafe { self.node_unchecked_mut(right_row) }.left = Some(row);

        self.rotate_common(row, right_row);
    }

    fn rotate_right(&mut self, row: NonZeroU32) {
        let left_row = unsafe { self.node_unchecked(row) }.left.unwrap();
        let left_right = unsafe { self.node_unchecked(left_row) }.right;

        unsafe { self.node_unchecked_mut(row) }.left = left_right;
        if let Some(left) = left_right {
            unsafe { self.node_unchecked_mut(left) }.parent = Some(row);
        }
        unsafe { self.node_unchecked_mut(left_row) }.right = Some(row);

        self.rotate_common(row, left_row);
    }
}
