use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    pub(crate) fn balance(&mut self, row: NonZeroU32) {
        let mut t: &AvltrieeNode<T> = unsafe { self.get_unchecked(row) };
        while t.parent != 0 {
            let u_row = unsafe { NonZeroU32::new_unchecked(t.parent) };
            let u = unsafe { self.get_unchecked_mut(u_row) };

            let height_before_balance = u.height;

            let left_row = unsafe { NonZeroU32::new_unchecked(u.left) };
            let right_row = unsafe { NonZeroU32::new_unchecked(u.right) };

            let left = unsafe { self.get_unchecked_mut(left_row) };
            let right = unsafe { self.get_unchecked_mut(right_row) };

            let t_row = match left.height as isize - right.height as isize {
                2 => {
                    if if left.left != 0 {
                        unsafe {
                            self.get_unchecked(NonZeroU32::new_unchecked(left.left))
                                .height
                        }
                    } else {
                        0
                    } < if left.right != 0 {
                        unsafe {
                            self.get_unchecked(NonZeroU32::new_unchecked(left.right))
                                .height
                        }
                    } else {
                        0
                    } {
                        self.rotate_left(left_row);
                    }
                    self.rotate_right(u_row);
                    right_row.get()
                }
                -2 => {
                    if if right.left != 0 {
                        unsafe {
                            self.get_unchecked(NonZeroU32::new_unchecked(right.left))
                                .height
                        }
                    } else {
                        0
                    } > if right.right != 0 {
                        unsafe {
                            self.get_unchecked(NonZeroU32::new_unchecked(right.right))
                                .height
                        }
                    } else {
                        0
                    } {
                        self.rotate_right(right_row);
                    }
                    self.rotate_left(u_row);
                    left_row.get()
                }
                _ => {
                    self.calc_height(u_row);
                    u_row.get()
                }
            };
            if height_before_balance == u.height {
                break;
            }
            if let Some(r) = NonZeroU32::new(t_row) {
                t = unsafe { self.get_unchecked(r) };
            } else {
                break;
            }
        }
    }

    fn rotate_common(&mut self, row: NonZeroU32, child_row: NonZeroU32) {
        let node_parent = unsafe { self.get_unchecked(row) }.parent;

        self.replace_child(node_parent, row, child_row);

        self.calc_height(row);
        self.calc_height(child_row);

        unsafe { self.get_unchecked_mut(child_row) }.parent = node_parent;
        unsafe { self.get_unchecked_mut(row) }.parent = child_row.get();
    }

    fn rotate_left(&mut self, row: NonZeroU32) {
        let node = unsafe { self.get_unchecked_mut(row) };

        let right_row = unsafe { NonZeroU32::new_unchecked(node.right) };
        let right = unsafe { self.get_unchecked_mut(right_row) };

        node.right = right.left;
        if let Some(right) = NonZeroU32::new(node.right) {
            unsafe { self.get_unchecked_mut(right) }.parent = row.get();
        }
        right.left = row.get();

        self.rotate_common(row, right_row);
    }

    fn rotate_right(&mut self, row: NonZeroU32) {
        let node = unsafe { self.get_unchecked_mut(row) };

        let left_row = unsafe { NonZeroU32::new_unchecked(node.left) };
        let left = unsafe { self.get_unchecked_mut(left_row) };

        node.left = left.right;
        if let Some(left) = NonZeroU32::new(node.left) {
            unsafe { self.get_unchecked_mut(left) }.parent = row.get();
        }
        left.right = row.get();

        self.rotate_common(row, left_row);
    }
}
