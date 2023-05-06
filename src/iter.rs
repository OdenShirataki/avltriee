use std::cmp::Ordering;

use super::Avltriee;
use super::AvltrieeNode;

mod iter;
pub use iter::AvltrieeIter;

mod range_iter;
pub use range_iter::AvltrieeRangeIter;

#[derive(PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

pub struct AvlTrieeIterResult<'a, T> {
    index: isize,
    row: u32,
    node: &'a AvltrieeNode<T>,
}
impl<'a, T> AvlTrieeIterResult<'a, T> {
    pub fn index(&self) -> isize {
        self.index
    }
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn value(&self) -> &'a T {
        &self.node.value
    }
}

impl<T> Avltriee<T> {
    pub fn iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, Order::Asc)
    }
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, Order::Desc)
    }

    pub fn iter_by<'a, F>(&'a self, search: F) -> AvltrieeRangeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        AvltrieeRangeIter::by(&self, self.search_eq(search))
    }

    pub fn iter_from<'a, F>(&'a self, search: F) -> AvltrieeRangeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        AvltrieeRangeIter::from(&self, self.search_ge(search))
    }

    pub fn iter_to<'a, F>(&'a self, search_from: F) -> AvltrieeRangeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        AvltrieeRangeIter::to(&self, self.search_le(search_from))
    }

    pub fn iter_range<'a, S, E>(&'a self, start: S, end: E) -> AvltrieeRangeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        if let Some(range) = self.search_range(start, end) {
            AvltrieeRangeIter::new(self, range.start, range.end)
        } else {
            AvltrieeRangeIter::empty(self)
        }
    }

    unsafe fn next(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
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
    unsafe fn retroactive(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        if self.offset(parent).right == c {
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

    unsafe fn next_desc(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
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
    unsafe fn retroactive_desc(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        if self.offset(parent).left == c {
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
}
