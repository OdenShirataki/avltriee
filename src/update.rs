use std::{cmp::Ordering, mem::size_of};

use libc::c_void;

use super::{Avltriee, AvltrieeNode, Found};

impl<T> Avltriee<T> {
    pub fn init_node(&mut self, data: T, root: u32) {
        (**self.node_list).parent = root;
        *unsafe { self.offset_mut(root) } = AvltrieeNode::new(1, 0, data);
    }

    pub unsafe fn update(&mut self, row: u32, data: T)
    where
        T: Ord,
    {
        if if let Some(n) = self.node(row) {
            if n.value.cmp(&data) != Ordering::Equal {
                self.delete(row);
                true
            } else {
                false
            }
        } else {
            true
        } {
            let found = self.search(|v| v.cmp(&data));
            if found.ord == Ordering::Equal && found.row != 0 {
                self.update_same(row, found.row);
            } else {
                self.update_unique(row, data, found);
                if self.root() == 0 {
                    self.set_root(row);
                }
            }
        }
    }

    pub unsafe fn update_unique(&mut self, row: u32, data: T, found: Found) {
        *self.offset_mut(row) = AvltrieeNode::new(row, found.row, data);
        if found.row > 0 {
            let p = self.offset_mut(found.row);
            if found.ord == Ordering::Greater {
                p.left = row;
            } else {
                p.right = row;
            }
            self.balance(found.row);
        }
    }
    pub unsafe fn update_same(&mut self, new_row: u32, vertex_row: u32) {
        let mut vertex = self.offset_mut(vertex_row);
        let mut new_vertex = self.offset_mut(new_row);
        libc::memcpy(
            new_vertex as *mut AvltrieeNode<T> as *mut c_void,
            vertex as *const AvltrieeNode<T> as *const c_void,
            size_of::<AvltrieeNode<T>>(),
        );
        if new_vertex.parent == 0 {
            self.set_root(new_row);
        } else {
            let mut parent = self.offset_mut(new_vertex.parent);
            if parent.left == vertex_row {
                parent.left = new_row;
            } else {
                parent.right = new_row;
            }
        }
        vertex.parent = new_row;
        new_vertex.same = vertex_row;
        if new_vertex.left != 0 {
            self.offset_mut(new_vertex.left).parent = new_row;
        }
        if new_vertex.right != 0 {
            self.offset_mut(new_vertex.right).parent = new_row;
        }
        vertex.left = 0;
        vertex.right = 0;
    }

    pub unsafe fn delete(&mut self, target_row: u32) {
        let remove_target = self.offset_mut(target_row);
        let height = remove_target.height;
        if height > 0 {
            let row_parent = remove_target.parent;
            let row_same = remove_target.same;
            let mut parent = self.offset_mut(row_parent);
            if row_parent != 0 && parent.same == target_row {
                parent.same = row_same;
                if row_same != 0 {
                    self.offset_mut(row_same).parent = row_parent;
                }
            } else {
                let row_left = remove_target.left;
                let row_right = remove_target.right;
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
                                self.remove_intermediate(remove_target);
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
                            self.remove_intermediate(remove_target);
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
            remove_target.height = 0;
        }
    }

    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }

    unsafe fn calc_height(&mut self, vertex_row: u32) {
        let mut vertex = self.offset_mut(vertex_row);
        vertex.height = std::cmp::max(
            self.offset(vertex.left).height,
            self.offset(vertex.right).height,
        ) + 1;
    }

    unsafe fn balance(&mut self, vertex_row: u32) {
        let mut vertex_row = vertex_row;
        loop {
            let mut vertex = self.offset_mut(vertex_row);

            let mut parent_row = vertex.parent;

            let left_row = vertex.left;
            let right_row = vertex.right;

            let mut left = self.offset_mut(left_row);
            let mut right = self.offset_mut(right_row);

            let diff = left.height as isize - right.height as isize;
            if diff.abs() >= 2 {
                let high_is_left = diff > 0;

                let new_vertex_row = if high_is_left {
                    self.max(left_row)
                } else {
                    self.min(right_row)
                };
                let new_vertex = self.offset_mut(new_vertex_row);
                let new_vertex_old_parent = new_vertex.parent;
                vertex.parent = new_vertex_row;
                new_vertex.parent = parent_row;
                if parent_row == 0 {
                    self.set_root(new_vertex_row);
                } else {
                    let parent = self.offset_mut(parent_row);
                    if parent.left == vertex_row {
                        parent.left = new_vertex_row;
                    } else {
                        parent.right = new_vertex_row;
                    }
                }
                if high_is_left {
                    new_vertex.right = vertex_row;
                    vertex.left = 0;
                    if new_vertex_row == left_row {
                        vertex = self.offset_mut(left_row);
                        left = self.offset_mut(vertex.left);
                        right = self.offset_mut(vertex_row);

                        self.calc_height(vertex.left);
                    } else {
                        let new_left_row = self.min(new_vertex_row);
                        let new_left = self.offset_mut(new_left_row);
                        new_left.left = left_row;

                        left.parent = new_left_row;
                        self.offset_mut(new_vertex_old_parent).right = 0;

                        self.calc_height(left_row);

                        left = self.offset_mut(vertex.left);

                        parent_row = new_vertex_old_parent;
                    }
                    self.calc_height(vertex_row);
                } else {
                    new_vertex.left = vertex_row;
                    vertex.right = 0;
                    if new_vertex_row == right_row {
                        vertex = self.offset_mut(right_row);
                        left = self.offset_mut(vertex_row);
                        right = self.offset_mut(vertex.right);

                        self.calc_height(vertex.right);
                    } else {
                        let new_right_row = self.max(new_vertex_row);
                        let new_right = self.offset_mut(new_right_row);
                        new_right.right = right_row;

                        right.parent = new_right_row;
                        self.offset_mut(new_vertex_old_parent).left = 0;

                        self.calc_height(right_row);

                        right = self.offset_mut(vertex.right);

                        parent_row = new_vertex_old_parent;
                    }
                    self.calc_height(vertex_row);
                }
            }

            vertex.height = std::cmp::max(left.height, right.height) + 1;
            vertex_row = parent_row;
            if vertex_row == 0 {
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
    unsafe fn remove_intermediate(&mut self, remove_target: &mut AvltrieeNode<T>) -> (u32, u32) {
        let left_max_row = self.max(remove_target.left);
        let mut left_max = self.offset_mut(left_max_row);
        let left_max_parent_row = left_max.parent;
        let mut left_max_parent = self.offset_mut(left_max_parent_row);

        if remove_target.left != left_max_row {
            left_max_parent.right = left_max.left;
            if left_max_parent.right != 0 {
                self.offset_mut(left_max_parent.right).parent = left_max_parent_row;
            }
            left_max.left = remove_target.left;
            if left_max.left != 0 {
                self.offset_mut(left_max.left).parent = left_max_row;
            }
        }

        left_max.parent = remove_target.parent;
        left_max.right = remove_target.right;

        let mut right = self.offset_mut(remove_target.right);
        right.parent = left_max_row;

        (left_max_row, left_max_parent_row)
    }
}
