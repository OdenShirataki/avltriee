use std::{
    cmp::{Ord, Ordering},
    mem::ManuallyDrop,
};

mod iter;
pub use iter::{AvltrieeIter, AvltrieeRangeIter};

mod node;
pub use node::AvltrieeNode;

mod update;
pub use update::Removed;

pub struct Found {
    row: u32,
    ord: Ordering,
}
impl Found {
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn ord(&self) -> Ordering {
        self.ord
    }
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

    pub fn root(&self) -> u32 {
        self.node_list.parent
    }

    pub fn search(&self, value: &T) -> Found
    where
        T: Ord,
    {
        let mut row = self.root();
        let mut ord = Ordering::Equal;

        while row != 0 {
            let p = unsafe { self.offset(row) };
            ord = value.cmp(&p.value);
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    row = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    row = p.right;
                }
            }
        }
        Found { row, ord }
    }

    pub fn search_cb<F>(&self, ord_cb: F) -> Found
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut ord = Ordering::Equal;
        while row != 0 {
            let p = unsafe { self.offset(row) };
            ord = ord_cb(&p.value);
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    row = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    row = p.right;
                }
            }
        }
        Found { row, ord }
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
