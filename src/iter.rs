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
        self.node.value()
    }
}

impl<T> Avltriee<T> {
    pub fn iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, Order::Asc)
    }
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, Order::Desc)
    }

    pub fn iter_by_value<'a>(&'a self, value: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value(&self, value)
    }
    pub fn iter_by_value_from<'a>(&'a self, from: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value_from(&self, from)
    }
    pub fn iter_by_value_to<'a>(&'a self, to: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value_to(&self, to)
    }
    pub fn iter_by_value_from_to<'a>(&'a self, from: &'a T, to: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value_from_to(&self, from, to)
    }
    pub fn iter_by_row_from_to(&self, from: u32, to: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, from, to)
    }
    pub fn iter_by_row_from(&self, from: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, from, unsafe { self.max(self.root()) })
    }
    pub fn iter_by_row_to(&self, to: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, unsafe { self.min(self.root()) }, to)
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
