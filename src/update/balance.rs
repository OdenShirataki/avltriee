use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T: Copy> Avltriee<T> {
    #[inline(always)]
    pub(crate) unsafe fn balance(&mut self, row: NonZeroU32) {
        let mut t_row = row.get();
        let mut t = self.offset(t_row);
        while t.parent != 0 {
            let u_row = t.parent;
            let u = self.offset_mut(u_row);

            let height_before_balance = u.height;

            let left = self.offset_mut(u.left);
            let right = self.offset_mut(u.right);

            t_row = match left.height as isize - right.height as isize {
                2 => {
                    if self.offset(left.left).height < self.offset(left.right).height {
                        self.rotate_left(left, NonZeroU32::new_unchecked(u.left));
                    }
                    self.rotate_right(u, NonZeroU32::new_unchecked(u_row));
                    u.right
                }
                -2 => {
                    if self.offset(right.left).height > self.offset(right.right).height {
                        self.rotate_right(right, NonZeroU32::new_unchecked(u.right));
                    }
                    self.rotate_left(u, NonZeroU32::new_unchecked(u_row));
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
            t = self.offset(t_row);
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    fn rotate_left(&mut self, node: &mut AvltrieeNode<T>, row: NonZeroU32) {
        let right_row = node.right;
        let right = unsafe { self.offset_mut(right_row) };

        node.right = right.left;
        self.set_parent(node.right, row.get());
        right.left = row.get();

        self.rotate_common(node, row, right, unsafe {
            NonZeroU32::new_unchecked(right_row)
        });
    }

    #[inline(always)]
    fn rotate_right(&mut self, node: &mut AvltrieeNode<T>, row: NonZeroU32) {
        let left_row = node.left;
        let left = unsafe { self.offset_mut(left_row) };

        node.left = left.right;
        self.set_parent(node.left, row.get());
        left.right = row.get();

        self.rotate_common(node, row, left, unsafe {
            NonZeroU32::new_unchecked(left_row)
        });
    }
}
