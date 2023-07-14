use std::cmp::Ordering;

use anyhow::Result;

use super::{Avltriee, AvltrieeNode, Found};

pub trait AvltrieeHolder<T, I> {
    fn triee(&self) -> &Avltriee<T>;
    fn triee_mut(&mut self) -> &mut Avltriee<T>;
    fn cmp(&self, left: &T, right: &I) -> Ordering;
    fn search_end(&self, input: &I) -> Found;
    fn value(&mut self, input: I) -> Result<T>;
    fn delete_before_update(&mut self, row: u32, delete_node: &T) -> Result<()>;
}

impl<T> AvltrieeHolder<T, T> for Avltriee<T>
where
    T: Ord,
{
    fn triee(&self) -> &Avltriee<T> {
        self
    }
    fn triee_mut(&mut self) -> &mut Avltriee<T> {
        self
    }
    fn cmp(&self, left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }
    fn search_end(&self, input: &T) -> Found {
        self.search_end(|v| v.cmp(input))
    }
    fn value(&mut self, input: T) -> Result<T> {
        Ok(input)
    }
    fn delete_before_update(&mut self, row: u32, _: &T) -> Result<()> {
        unsafe {
            self.delete(row);
        }
        Ok(())
    }
}

impl<T> Avltriee<T> {
    pub unsafe fn update(&mut self, row: u32, value: T) -> Result<()>
    where
        T: Ord + Clone,
    {
        Self::update_holder(self, row, value)
    }

    pub unsafe fn update_holder<H, I>(holder: &mut H, row: u32, input: I) -> Result<()>
    where
        T: Clone,
        H: AvltrieeHolder<T, I>,
    {
        if let Some(node) = holder.triee().node(row) {
            if holder.cmp(node, &input) == Ordering::Equal {
                return Ok(()); //update value eq exists value
            }
            holder.delete_before_update(row, node)?;
        }
        let found = holder.search_end(&input);
        if found.ord == Ordering::Equal && found.row != 0 {
            holder.triee_mut().update_same(row, found.row);
        } else {
            let value = holder.value(input)?;
            holder.triee_mut().insert_unique(row, value, found);
        }
        Ok(())
    }

    pub unsafe fn insert_unique(&mut self, row: u32, value: T, found: Found) {
        *self.offset_mut(row) = AvltrieeNode::new(row, found.row, value);
        if found.row == 0 {
            self.set_root(row);
        } else {
            assert!(
                found.ord != Ordering::Equal,
                "Avltriee.insert_unique : {:?}",
                &found
            );
            let p = self.offset_mut(found.row);
            if found.ord == Ordering::Greater {
                p.left = row;
            } else {
                p.right = row;
            }
            self.balance(true, row);
        }
    }

    pub(crate) unsafe fn update_same(&mut self, row: u32, same: u32)
    where
        T: Clone,
    {
        let same_node = self.offset_mut(same);
        let node = self.offset_mut(row);

        *node = same_node.clone();

        if node.parent == 0 {
            self.set_root(row);
        } else {
            let parent = self.offset_mut(node.parent);
            if parent.left == same {
                parent.left = row;
            } else {
                parent.right = row;
            }
        }
        same_node.parent = row;
        node.same = same;
        if node.left != 0 {
            self.offset_mut(node.left).parent = row;
        }
        if node.right != 0 {
            self.offset_mut(node.right).parent = row;
        }
        same_node.left = 0;
        same_node.right = 0;
    }

    pub unsafe fn delete(&mut self, target_row: u32) {
        let delete_node = self.offset_mut(target_row);
        let height = delete_node.height;
        if height > 0 {
            let row_parent = delete_node.parent;
            let row_same = delete_node.same;
            let mut parent = self.offset_mut(row_parent);
            if row_parent != 0 && parent.same == target_row {
                parent.same = row_same;
                if row_same != 0 {
                    self.offset_mut(row_same).parent = row_parent;
                }
            } else {
                let row_left = delete_node.left;
                let row_right = delete_node.right;
                if row_same != 0 {
                    let same = self.offset_mut(row_same);
                    same.parent = row_parent;
                    same.left = row_left;
                    same.right = row_right;
                    same.height = height;
                    if same.left != 0 {
                        self.offset_mut(same.left).parent = row_same;
                    }
                    if same.right != 0 {
                        self.offset_mut(same.right).parent = row_same;
                    }
                    if row_parent == 0 {
                        self.set_root(row_same);
                    } else {
                        Self::join_intermediate(
                            &mut self.offset_mut(same.parent),
                            target_row,
                            row_same,
                        );
                    }
                } else if row_parent == 0 {
                    if row_left == 0 && row_right == 0 {
                        self.set_root(0);
                    } else {
                        let balance_row = if row_left == 0 {
                            self.set_root(row_right);
                            self.offset_mut(row_right).parent = 0;
                            row_right
                        } else if row_right == 0 {
                            self.set_root(row_left);
                            self.offset_mut(row_left).parent = 0;
                            row_left
                        } else {
                            let (left_max_row, left_max_parent_row) =
                                self.delete_intermediate(delete_node);
                            self.set_root(left_max_row);
                            if left_max_parent_row == target_row {
                                self.offset_mut(left_max_parent_row).parent = left_max_row;
                                left_max_row
                            } else {
                                left_max_parent_row
                            }
                        };
                        self.balance(false, balance_row);
                    }
                } else {
                    let balance_row = if row_left == 0 && row_right == 0 {
                        Self::join_intermediate(&mut parent, target_row, row_same);
                        row_parent
                    } else if row_left == 0 {
                        Self::join_intermediate(parent, target_row, row_right);
                        self.offset_mut(row_right).parent = row_parent;
                        row_parent
                    } else if row_right == 0 {
                        Self::join_intermediate(parent, target_row, row_left);
                        self.offset_mut(row_left).parent = row_parent;
                        row_parent
                    } else {
                        let (left_max_row, left_max_parent_row) =
                            self.delete_intermediate(delete_node);
                        if parent.right == target_row {
                            parent.right = left_max_row;
                        } else {
                            parent.left = left_max_row;
                        }
                        if left_max_parent_row == target_row {
                            left_max_row
                        } else {
                            left_max_parent_row
                        }
                    };
                    self.balance(false, balance_row);
                }
            }
            delete_node.height = 0;
        }
    }

    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }

    fn calc_height(&mut self, row: u32) {
        let node = unsafe { self.offset_mut(row) };
        node.height = std::cmp::max(
            unsafe { self.offset(node.left) }.height,
            unsafe { self.offset(node.right) }.height,
        ) + 1;
    }

    fn balance(&mut self, is_insert: bool, row: u32) {
        let mut t_row = row;
        let mut t = unsafe { self.offset(t_row) };
        while t.parent != 0 {
            let mut u_row = t.parent;
            let u = unsafe { self.offset(u_row) };

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
                    self.calc_height(u_row);
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
                    self.calc_height(u_row);
                }
            }
            if height_before_balance == unsafe { self.offset(u_row) }.height {
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

    fn join_intermediate(parent: &mut AvltrieeNode<T>, remove_target_row: u32, child_row: u32) {
        if parent.right == remove_target_row {
            parent.right = child_row;
        } else if parent.left == remove_target_row {
            parent.left = child_row;
        }
    }
    unsafe fn delete_intermediate(&mut self, delete_node: &mut AvltrieeNode<T>) -> (u32, u32) {
        let left_max_row = self.max(delete_node.left);
        let left_max = self.offset_mut(left_max_row);
        let left_max_parent_row = left_max.parent;
        let left_max_parent = self.offset_mut(left_max_parent_row);

        if delete_node.left != left_max_row {
            left_max_parent.right = left_max.left;
            if left_max_parent.right != 0 {
                self.offset_mut(left_max_parent.right).parent = left_max_parent_row;
            }
            left_max.left = delete_node.left;
            if left_max.left != 0 {
                self.offset_mut(left_max.left).parent = left_max_row;
            }
        }

        left_max.parent = delete_node.parent;
        left_max.right = delete_node.right;

        let right = self.offset_mut(delete_node.right);
        right.parent = left_max_row;

        (left_max_row, left_max_parent_row)
    }
}
