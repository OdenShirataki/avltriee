use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    pub(crate) fn balance(&mut self, row: NonZeroU32) {
        let mut t: &AvltrieeNode<T> = unsafe { self.get_unchecked(row) };
        while t.parent != 0 {
            let u_row = t.parent;
            let u = unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(u_row)) };

            let height_before_balance = u.height;

            let left = unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(u.left)) };
            let right = unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(u.right)) };

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
                        self.rotate_left(left, unsafe { NonZeroU32::new_unchecked(u.left) });
                    }
                    self.rotate_right(u, unsafe { NonZeroU32::new_unchecked(u_row) });
                    u.right
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
                        self.rotate_right(right, unsafe { NonZeroU32::new_unchecked(u.right) });
                    }
                    self.rotate_left(u, unsafe { NonZeroU32::new_unchecked(u_row) });
                    u.left
                }
                _ => {
                    self.calc_height_node(u);
                    u_row
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

    fn rotate_common(
        &mut self,
        node: &mut AvltrieeNode<T>,
        row: NonZeroU32,
        child_node: &mut AvltrieeNode<T>,
        child_row: NonZeroU32,
    ) {
        self.change_row(node, row, child_row);

        self.calc_height(row);
        self.calc_height(child_row);

        child_node.parent = node.parent;
        node.parent = child_row.get();
    }

    fn rotate_left(&mut self, node: &mut AvltrieeNode<T>, row: NonZeroU32) {
        let right_row = node.right;
        let right = unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(right_row)) };

        node.right = right.left;
        if let Some(right) = NonZeroU32::new(node.right) {
            self.set_parent(right, row.get());
        }
        right.left = row.get();

        self.rotate_common(node, row, right, unsafe {
            NonZeroU32::new_unchecked(right_row)
        });
    }

    fn rotate_right(&mut self, node: &mut AvltrieeNode<T>, row: NonZeroU32) {
        let left_row = node.left;
        let left = unsafe { self.get_unchecked_mut(NonZeroU32::new_unchecked(left_row)) };

        node.left = left.right;
        if let Some(left) = NonZeroU32::new(node.left) {
            self.set_parent(left, row.get());
        }
        left.right = row.get();

        self.rotate_common(node, row, left, unsafe {
            NonZeroU32::new_unchecked(left_row)
        });
    }
}
