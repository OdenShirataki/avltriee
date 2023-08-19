use crate::Avltriee;

impl<T> Avltriee<T> {
    pub(crate) fn balance(&mut self, is_insert: bool, row: u32) {
        let mut t_row = row;
        let mut t = unsafe { self.offset(t_row) };
        while t.parent != 0 {
            let mut u_row = t.parent;
            let u = unsafe { self.offset_mut(u_row) };

            let height_before_balance = u.height;

            let left = unsafe { self.offset(u.left) };
            let right = unsafe { self.offset(u.right) };
            let bias = left.height as isize - right.height as isize;
            if (u.left == t_row) == is_insert {
                if bias == 2 {
                    u_row = if unsafe { self.offset(left.left) }.height as isize
                        - unsafe { self.offset(left.right) }.height as isize
                        >= 0
                    {
                        self.rotate_right(u_row)
                    } else {
                        self.rotate_left_right(u_row)
                    };
                } else {
                    self.calc_height_node(u);
                }
            } else {
                if bias == -2 {
                    u_row = if unsafe { self.offset(right.left) }.height as isize
                        - unsafe { self.offset(right.right) }.height as isize
                        <= 0
                    {
                        self.rotate_left(u_row)
                    } else {
                        self.rotate_right_left(u_row)
                    };
                } else {
                    self.calc_height_node(u);
                }
            }
            if height_before_balance == u.height {
                break;
            }
            t_row = u_row;
            t = unsafe { self.offset(t_row) };
        }
    }

    fn rotate_left_right(&mut self, row: u32) -> u32 {
        self.rotate_left(unsafe { self.offset(row) }.left);
        self.rotate_right(row)
    }
    fn rotate_right_left(&mut self, row: u32) -> u32 {
        self.rotate_right(unsafe { self.offset(row) }.right);
        self.rotate_left(row)
    }
    fn rotate_left(&mut self, row: u32) -> u32 {
        assert!(row != 0, "row is 0");
        let v = unsafe { self.offset_mut(row) };

        let right_row = v.right;
        assert!(right_row != 0, "row is 0");
        let right = unsafe { self.offset_mut(right_row) };

        v.right = right.left;

        if v.right != 0 {
            unsafe { self.offset_mut(v.right) }.parent = row;
        }
        right.left = row;
        if v.parent == 0 {
            self.set_root(right_row);
        } else {
            let parent = unsafe { self.offset_mut(v.parent) };
            if parent.left == row {
                parent.left = right_row;
            } else {
                parent.right = right_row;
            }
        }
        self.calc_height(row);
        self.calc_height(right_row);

        right.parent = v.parent;
        v.parent = right_row;

        right_row
    }
    fn rotate_right(&mut self, row: u32) -> u32 {
        assert!(row != 0, "row is 0");
        let v = unsafe { self.offset_mut(row) };

        let left_row = v.left;
        assert!(left_row != 0, "row is 0");
        let left = unsafe { self.offset_mut(left_row) };

        v.left = left.right;
        if v.left != 0 {
            unsafe { self.offset_mut(v.left) }.parent = row;
        }
        left.right = row;
        if v.parent == 0 {
            self.set_root(left_row);
        } else {
            let parent = unsafe { self.offset_mut(v.parent) };
            if parent.left == row {
                parent.left = left_row;
            } else {
                parent.right = left_row;
            }
        }
        self.calc_height(row);
        self.calc_height(left_row);

        left.parent = v.parent;
        v.parent = left_row;

        left_row
    }
}
