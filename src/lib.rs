use std::{cmp::Ordering, mem::ManuallyDrop, ops::Range};

mod iter;
pub use iter::AvltrieeIter;

mod node;
pub use node::AvltrieeNode;

mod update;
pub use update::Removed;

mod found;
pub use found::Found;

pub struct Avltriee<T> {
    node_list: ManuallyDrop<Box<AvltrieeNode<T>>>,
}
impl<T> Avltriee<T> {
    pub fn new(node_list: *mut AvltrieeNode<T>) -> Avltriee<T> {
        Avltriee {
            node_list: ManuallyDrop::new(unsafe { Box::from_raw(node_list) }),
        }
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
            Some(&v.value)
        } else {
            None
        }
    }
    pub unsafe fn value_unchecked<'a>(&self, row: u32) -> &'a T {
        &self.offset(row).value
    }

    pub fn root(&self) -> u32 {
        self.node_list.parent
    }

    pub fn search<F>(&self, compare: F) -> Found
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut ord = Ordering::Equal;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            ord = compare(&node.value);
            match ord {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        Found { row, ord }
    }

    pub fn search_eq<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let found = self.search(compare);
        if found.ord == Ordering::Equal {
            found.row
        } else {
            0
        }
    }
    pub fn search_gt<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            match compare(&node.value) {
                Ordering::Greater => {
                    if node.left == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.left;
                }
                Ordering::Equal => {
                    return if node.right != 0 {
                        unsafe { self.min(node.right) }
                    } else {
                        if unsafe { self.offset(node.parent) }.left == row {
                            node.parent
                        } else {
                            keep
                        }
                    };
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        keep
    }
    pub fn search_ge<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            match compare(&node.value) {
                Ordering::Greater => {
                    if node.left == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.left;
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        keep
    }
    pub fn search_lt<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            match compare(&node.value) {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    return if node.left != 0 {
                        unsafe { self.max(node.left) }
                    } else {
                        if unsafe { self.offset(node.parent) }.right == row {
                            node.parent
                        } else {
                            keep
                        }
                    };
                }
                Ordering::Less => {
                    if node.right == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.right;
                }
            }
        }
        keep
    }
    pub fn search_le<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            match compare(&node.value) {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.right;
                }
            }
        }
        keep
    }
    pub fn search_range<S, E>(&self, compare_ge: S, compare_le: E) -> Option<Range<u32>>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut start = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            let ord = compare_ge(&node.value);
            match ord {
                Ordering::Greater => {
                    start = row;
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    start = row;
                    break;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        if start == 0 || compare_le(&unsafe { self.offset(start) }.value) == Ordering::Greater {
            return None;
        }

        row = self.root();
        let mut end = 0;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            match compare_le(&node.value) {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    end = row;
                    break;
                }
                Ordering::Less => {
                    end = row;
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        if end == 0 {
            return None;
        }
        Some(Range { start, end })
    }

    unsafe fn offset<'a>(&self, offset: u32) -> &'a AvltrieeNode<T> {
        &*(&**self.node_list as *const AvltrieeNode<T>).offset(offset as isize)
    }
    unsafe fn offset_mut<'a>(&mut self, offset: u32) -> &'a mut AvltrieeNode<T> {
        &mut *(&mut **self.node_list as *mut AvltrieeNode<T>).offset(offset as isize)
    }

    unsafe fn min(&self, t: u32) -> u32 {
        let l = self.offset(t).left;
        if l == 0 {
            t
        } else {
            self.min(l)
        }
    }
    unsafe fn max(&self, t: u32) -> u32 {
        let r = self.offset(t).right;
        if r == 0 {
            t
        } else {
            self.max(r)
        }
    }
}
