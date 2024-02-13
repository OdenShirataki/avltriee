use std::{cmp::Ordering, num::NonZeroU32, ops::Range};

use crate::{Avltriee, AvltrieeAllocator};

pub(crate) type Edge = (Option<NonZeroU32>, Ordering);

pub trait AvltrieeSearch<T, I: ?Sized, A: AvltrieeAllocator<T>>: AsRef<Avltriee<T, I, A>> {
    fn cmp(left: &I, right: &I) -> Ordering;
    fn invert<'a>(&'a self, value: &'a T) -> &I;

    /// Finds the edge of a node from the specified value.
    fn edge(&self, value: &I) -> Edge {
        let triee = self.as_ref();
        let mut row: Option<NonZeroU32> = triee.root();
        let mut ord = Ordering::Equal;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            ord = Self::cmp(self.invert(node), value);
            match ord {
                Ordering::Greater => {
                    if node.left.is_some() {
                        row = node.left;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        row = node.right;
                    } else {
                        break;
                    }
                }
            }
        }
        (row, ord)
    }

    /// Search row of a value.
    fn row(&self, value: &I) -> Option<NonZeroU32> {
        let edge = self.edge(value);
        (edge.1 == Ordering::Equal).then(|| edge.0).flatten()
    }

    /// Returns the value of the specified row. Returns None if the row does not exist.
    fn value<'a>(&'a self, row: NonZeroU32) -> Option<&I>
    where
        A: 'a,
        T: 'a,
    {
        self.as_ref().node(row).map(|v| self.invert(v))
    }

    /// Returns the value of the specified row.
    unsafe fn value_unchecked<'a>(&'a self, row: NonZeroU32) -> &I
    where
        A: 'a,
        T: 'a,
    {
        self.invert(self.as_ref().node_unchecked(row))
    }

    /// Search >= value.
    fn ge(&self, value: &I) -> Option<NonZeroU32> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            match Self::cmp(self.invert(node), value) {
                Ordering::Greater => {
                    if node.left.is_some() {
                        keep = row;
                        row = node.left;
                    } else {
                        return row;
                    }
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        row = node.right;
                    } else {
                        break;
                    }
                }
            }
        }
        keep
    }

    /// Search <= value.
    fn le(&self, value: &I) -> Option<NonZeroU32> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            match Self::cmp(self.invert(node), value) {
                Ordering::Greater => {
                    if node.left.is_some() {
                        row = node.left;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        keep = row;
                        row = node.right;
                    } else {
                        return row;
                    }
                }
            }
        }
        keep
    }

    /// Search > value.
    fn gt(&self, value: &I) -> Option<NonZeroU32> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            match Self::cmp(self.invert(node), value) {
                Ordering::Greater => {
                    if node.left.is_some() {
                        keep = row;
                        row = node.left;
                    } else {
                        return row;
                    }
                }
                Ordering::Equal => {
                    if node.right.is_some() {
                        return triee.min(node.right);
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { triee.node_unchecked(parent).left } == row {
                            return node.parent;
                        }
                    }
                    break;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        row = node.right;
                    } else {
                        break;
                    }
                }
            }
        }
        keep
    }

    /// Search < value.
    fn lt(&self, value: &I) -> Option<NonZeroU32> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            match Self::cmp(self.invert(node), value) {
                Ordering::Greater => {
                    if node.left.is_some() {
                        row = node.left;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    if node.left.is_some() {
                        return triee.max(node.left);
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { triee.node_unchecked(parent) }.right == row {
                            return node.parent;
                        }
                    }
                    break;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        keep = row;
                        row = node.right;
                    } else {
                        return row;
                    }
                }
            }
        }
        keep
    }

    /// Search with range value with custom ord.
    fn range(&self, start_value: &I, end_value: &I) -> Option<Range<NonZeroU32>> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut start = None;
        while let Some(row_inner) = row {
            let node = unsafe { triee.node_unchecked(row_inner) };
            match Self::cmp(self.invert(node), start_value) {
                Ordering::Greater => {
                    start = row;
                    if node.left.is_some() {
                        row = node.left;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    start = row;
                    break;
                }
                Ordering::Less => {
                    if node.right.is_some() {
                        row = node.right;
                    } else {
                        break;
                    }
                }
            }
        }
        if let Some(start) = start {
            if Self::cmp(
                self.invert(unsafe { triee.node_unchecked(start) }),
                end_value,
            ) != Ordering::Greater
            {
                row = triee.root();
                let mut end = None;
                while let Some(row_inner) = row {
                    let node = unsafe { triee.node_unchecked(row_inner) };
                    match Self::cmp(self.invert(node), end_value) {
                        Ordering::Greater => {
                            if node.left.is_some() {
                                row = node.left;
                            } else {
                                break;
                            }
                        }
                        Ordering::Equal => {
                            end = row;
                            break;
                        }
                        Ordering::Less => {
                            end = row;
                            if node.right.is_some() {
                                row = node.right;
                            } else {
                                break;
                            }
                        }
                    }
                }
                if let Some(end) = end {
                    return Some(Range { start, end });
                }
            }
        }
        None
    }
}
