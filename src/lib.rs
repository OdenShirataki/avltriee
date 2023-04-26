use std::{
    cmp::{Ord, Ordering},
    mem::ManuallyDrop,
};

mod iter;
pub use iter::{AvltrieeIter, AvltrieeRangeIter};

mod node;
pub use node::AvltrieeNode;

pub enum Removed<T> {
    Last(T),
    Remain,
    None,
}

pub struct Avltriee<T> {
    node_list: ManuallyDrop<Box<AvltrieeNode<T>>>,
}
impl<T> Avltriee<T> {
    pub fn new(node_list: *mut AvltrieeNode<T>) -> Avltriee<T> {
        Avltriee {
            node_list: ManuallyDrop::new(unsafe { Box::from_raw(node_list) }),
        }
    }
    pub unsafe fn update(&mut self, row: u32, data: T)
    where
        T: Ord + Clone + Default,
    {
        if if let Some(n) = self.node(row) {
            if n.value().cmp(&data) != Ordering::Equal {
                self.remove(row);
                true
            } else {
                false
            }
        } else {
            true
        } {
            let (ord, found_row) = self.search(&data);
            if ord == Ordering::Equal && found_row != 0 {
                self.update_same(found_row, row);
            } else {
                self.update_node(found_row, row, data, ord);
                if self.root() == 0 {
                    self.set_root(row);
                }
            }
        }
    }

    pub unsafe fn update_node(&mut self, origin: u32, target_row: u32, data: T, ord: Ordering) {
        *self.offset_mut(target_row) = AvltrieeNode::new(target_row, origin, data);
        if origin > 0 {
            let p = self.offset_mut(origin);
            if ord == Ordering::Less {
                p.left = target_row;
            } else {
                p.right = target_row;
            }
            self.balance(origin);
        }
    }
    pub unsafe fn update_same(&mut self, vertex_row: u32, new_row: u32)
    where
        T: Clone,
    {
        let mut vertex = self.offset_mut(vertex_row);
        let mut new_vertex = self.offset_mut(new_row);
        *new_vertex = vertex.clone();
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

    pub fn iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, iter::Order::Asc)
    }
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, iter::Order::Desc)
    }
    pub fn iter_by_value_from(&self, min_value: &T) -> AvltrieeIter<T>
    where
        T: Ord,
    {
        let (_, row) = self.search(min_value);
        AvltrieeIter::begin_at(&self, row, iter::Order::Asc)
    }
    pub fn iter_by_value_to<'a>(&'a self, max_value: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value_max(&self, max_value)
    }
    pub fn iter_by_value_from_to<'a>(
        &'a self,
        min_value: &'a T,
        max_value: &'a T,
    ) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value(&self, min_value, max_value)
    }
    pub fn iter_by_row_from_to(&self, from: u32, to: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(
            &self,
            if let Some(_) = unsafe { self.node(from) } {
                from
            } else {
                0
            },
            to,
        )
    }
    pub fn iter_by_row_from(&self, from: u32) -> AvltrieeIter<T> {
        AvltrieeIter::begin_at(
            &self,
            if let Some(_) = unsafe { self.node(from) } {
                from
            } else {
                0
            },
            iter::Order::Asc,
        )
    }
    pub fn iter_by_row_to(&self, end: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, unsafe { self.min(self.root()) }, end)
    }
    pub unsafe fn node<'a>(&self, row: u32) -> Option<&'a AvltrieeNode<T>> {
        let node = self.offset(row);
        if node.height > 0 {
            Some(node)
        } else {
            None
        }
    }
    pub unsafe fn value<'a>(&self, row: u32) -> Option<&'a T> {
        if let Some(v) = self.node(row) {
            Some(&v.value())
        } else {
            None
        }
    }
    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }
    pub fn root(&self) -> u32 {
        self.node_list.parent
    }
    pub fn init_node(&mut self, data: T, root: u32)
    where
        T: Default,
    {
        **self.node_list = AvltrieeNode::new(0, root, T::default());
        *unsafe { self.offset_mut(root) } = AvltrieeNode::new(1, 0, data);
    }

    pub(crate) unsafe fn offset<'a>(&self, offset: u32) -> &'a AvltrieeNode<T> {
        &*(&**self.node_list as *const AvltrieeNode<T>).offset(offset as isize)
    }
    pub(crate) unsafe fn offset_mut<'a>(&mut self, offset: u32) -> &'a mut AvltrieeNode<T> {
        &mut *(&mut **self.node_list as *mut AvltrieeNode<T>).offset(offset as isize)
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

    pub unsafe fn remove(&mut self, target_row: u32) -> Removed<T>
    where
        T: Default + Clone,
    {
        let mut ret = Removed::Remain;
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
                    ret = Removed::Last(remove_target.value().clone());
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
                    ret = Removed::Last(remove_target.value().clone());
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
            remove_target.reset();
        }
        ret
    }

    unsafe fn calc_height(&mut self, vertex_row: u32) {
        let mut vertex = self.offset_mut(vertex_row);

        let left = self.offset(vertex.left);
        let right = self.offset(vertex.right);

        vertex.height = std::cmp::max(left.height, right.height) + 1;
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

    pub fn search(&self, value: &T) -> (Ordering, u32)
    where
        T: Ord,
    {
        let mut origin = self.root();
        let mut ord = Ordering::Equal;

        while origin != 0 {
            let p = unsafe { self.offset(origin) };
            ord = value.cmp(&p.value());
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    origin = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    origin = p.right;
                }
            }
        }
        (ord, origin)
    }
    pub fn search_cb<F>(&self, ord_cb: F) -> (Ordering, u32)
    where
        F: Fn(&T) -> Ordering,
    {
        let mut origin = self.root();
        let mut ord = Ordering::Equal;
        while origin != 0 {
            let p = unsafe { self.offset(origin) };
            ord = ord_cb(&p.value());
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    origin = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    origin = p.right;
                }
            }
        }
        (ord, origin)
    }
    pub unsafe fn sames(&self, same_root: u32) -> Vec<u32> {
        let mut r = Vec::new();
        let mut t = same_root;
        loop {
            let same = self.offset(t).same;
            if same != 0 {
                r.push(same.into());
                t = same;
            } else {
                break;
            }
        }
        r
    }
    unsafe fn max(&self, t: u32) -> u32 {
        let r = self.offset(t).right;
        if r == 0 {
            t
        } else {
            self.max(r)
        }
    }
    unsafe fn min(&self, t: u32) -> u32 {
        let l = self.offset(t).left;
        if l == 0 {
            t
        } else {
            self.min(l)
        }
    }
    unsafe fn retroactive(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        let parent_node = self.offset(parent);
        if parent_node.right == c {
            if let Some(p) = self.retroactive(parent) {
                if p != c {
                    return Some(p);
                }
            }
        } else {
            return Some(parent);
        }
        None
    }
    pub(crate) unsafe fn next(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let mut current = c;
        let mut node = self.offset(current);

        if node.same != 0 {
            return Some((node.same, if same_branch == 0 { c } else { same_branch }));
        } else {
            if same_branch != 0 {
                current = same_branch;
                node = self.offset(same_branch);
            }
            let parent = node.parent;
            if node.right != 0 {
                return Some((self.min(node.right), 0));
            } else if parent != 0 {
                if self.offset(parent).left == current {
                    return Some((parent, 0));
                } else if let Some(i) = self.retroactive(parent) {
                    return Some((i, 0));
                }
            }
        }
        None
    }

    unsafe fn retroactive_desc(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        let parent_node = self.offset(parent);
        if parent_node.left == c {
            if let Some(p) = self.retroactive_desc(parent) {
                if p != c {
                    return Some(p);
                }
            }
        } else {
            return Some(parent);
        }
        None
    }
    pub(crate) unsafe fn next_desc(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let mut current = c;
        let mut node = self.offset(current);

        if node.same != 0 {
            return Some((node.same, if same_branch == 0 { c } else { same_branch }));
        } else {
            if same_branch != 0 {
                current = same_branch;
                node = self.offset(same_branch);
            }
            let parent = node.parent;
            if node.left != 0 {
                return Some((self.max(node.left), 0));
            } else if parent != 0 {
                if self.offset(parent).right == current {
                    return Some((parent, 0));
                } else if let Some(i) = self.retroactive_desc(parent) {
                    return Some((i, 0));
                }
            }
        }
        None
    }
}
