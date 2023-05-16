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
            self.balance(found.row);
        }
    }

    pub(crate) unsafe fn update_same(&mut self, row: u32, same: u32)
    where
        T: Clone,
    {
        let mut same_node = self.offset_mut(same);
        let mut node = self.offset_mut(row);

        *node = same_node.clone();

        if node.parent == 0 {
            self.set_root(row);
        } else {
            let mut parent = self.offset_mut(node.parent);
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
                        self.balance(balance_row);
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
                    self.balance(balance_row);
                }
            }
            delete_node.height = 0;
        }
    }

    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }

    unsafe fn calc_height(&mut self, row: u32) {
        let mut node = self.offset_mut(row);
        node.height = std::cmp::max(
            self.offset(node.left).height,
            self.offset(node.right).height,
        ) + 1;
    }

    unsafe fn balance(&mut self, row: u32) {
        let mut row = row;
        loop {
            let mut node = self.offset_mut(row);

            let mut parent_row = node.parent;

            let left_row = node.left;
            let right_row = node.right;

            let mut left = self.offset_mut(left_row);
            let mut right = self.offset_mut(right_row);

            let diff = left.height as isize - right.height as isize;
            if diff.abs() >= 2 {
                let high_is_left = diff > 0;

                let vertex_row = if high_is_left {
                    self.max(left_row)
                } else {
                    self.min(right_row)
                };
                let vertex_node = self.offset_mut(vertex_row);
                let vertex_parent = vertex_node.parent;
                node.parent = vertex_row;
                vertex_node.parent = parent_row;
                if parent_row == 0 {
                    self.set_root(vertex_row);
                } else {
                    let parent = self.offset_mut(parent_row);
                    if parent.left == row {
                        parent.left = vertex_row;
                    } else {
                        parent.right = vertex_row;
                    }
                }
                if high_is_left {
                    vertex_node.right = row;
                    node.left = 0;
                    if vertex_row == left_row {
                        node = self.offset_mut(left_row);
                        left = self.offset_mut(node.left);
                        right = self.offset_mut(row);

                        self.calc_height(node.left);
                    } else {
                        let new_left_row = self.min(vertex_row);
                        let new_left = self.offset_mut(new_left_row);
                        new_left.left = left_row;

                        left.parent = new_left_row;
                        self.offset_mut(vertex_parent).right = 0;

                        self.calc_height(left_row);

                        left = self.offset_mut(node.left);

                        parent_row = vertex_parent;
                    }
                    self.calc_height(row);
                } else {
                    vertex_node.left = row;
                    node.right = 0;
                    if vertex_row == right_row {
                        node = self.offset_mut(right_row);
                        left = self.offset_mut(row);
                        right = self.offset_mut(node.right);

                        self.calc_height(node.right);
                    } else {
                        let new_right_row = self.max(vertex_row);
                        let new_right = self.offset_mut(new_right_row);
                        new_right.right = right_row;

                        right.parent = new_right_row;
                        self.offset_mut(vertex_parent).left = 0;

                        self.calc_height(right_row);

                        right = self.offset_mut(node.right);

                        parent_row = vertex_parent;
                    }
                    self.calc_height(row);
                }
            }

            node.height = std::cmp::max(left.height, right.height) + 1;
            row = parent_row;
            if row == 0 {
                break;
            }
        }
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
        let mut left_max = self.offset_mut(left_max_row);
        let left_max_parent_row = left_max.parent;
        let mut left_max_parent = self.offset_mut(left_max_parent_row);

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

        let mut right = self.offset_mut(delete_node.right);
        right.parent = left_max_row;

        (left_max_row, left_max_parent_row)
    }
}
