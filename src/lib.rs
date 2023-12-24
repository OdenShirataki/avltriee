mod head;
mod iter;
mod node;
mod update;

pub use iter::AvltrieeIter;
pub use node::AvltrieeNode;
pub use update::AvltrieeHolder;

use std::{cmp::Ordering, num::NonZeroU32, ops::Deref, ptr::NonNull};

#[derive(Debug)]
pub struct Found {
    row: u32,
    ord: Ordering,
}
impl Found {
    #[inline(always)]
    pub fn row(&self) -> u32 {
        self.row
    }

    #[inline(always)]
    pub fn ord(&self) -> Ordering {
        self.ord
    }
}

pub struct Avltriee<T> {
    node_list: NonNull<AvltrieeNode<T>>,
}

impl<T> Avltriee<T> {
    pub fn new(node_list: NonNull<AvltrieeNode<T>>) -> Avltriee<T> {
        Avltriee { node_list }
    }

    pub unsafe fn node<'a>(&self, row: NonZeroU32) -> Option<&'a AvltrieeNode<T>> {
        let node = self.offset(row.get());
        (node.height > 0).then_some(node)
    }

    pub unsafe fn value(&self, row: NonZeroU32) -> Option<&T> {
        self.node(row).map(|x| x.deref())
    }

    pub unsafe fn value_unchecked(&self, row: NonZeroU32) -> &T {
        self.offset(row.get())
    }

    pub fn search_end<F>(&self, cmp: F) -> Found
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut ord = Ordering::Equal;
        while row != 0 {
            let node = unsafe { self.offset(row) };
            ord = cmp(node);
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

    pub unsafe fn is_unique(&self, row: NonZeroU32) -> bool {
        let node = self.offset(row.get());
        node.same == 0 && (node.parent == 0 || self.offset(node.parent).same != row.get())
    }

    unsafe fn offset<'a>(&self, offset: u32) -> &'a AvltrieeNode<T> {
        &*self.node_list.as_ptr().offset(offset as isize)
    }

    unsafe fn offset_mut<'a>(&mut self, offset: u32) -> &'a mut AvltrieeNode<T> {
        &mut *self.node_list.as_ptr().offset(offset as isize)
    }

    fn min(&self, t: u32) -> u32 {
        let mut t = t;
        while t != 0 {
            let l = unsafe { self.offset(t) }.left;
            if l == 0 {
                break;
            }
            t = l;
        }
        t
    }

    fn max(&self, t: u32) -> u32 {
        let mut t = t;
        while t != 0 {
            let r = unsafe { self.offset(t) }.right;
            if r == 0 {
                break;
            }
            t = r;
        }
        t
    }
}
