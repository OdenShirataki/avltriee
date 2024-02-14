use std::{cmp::Ordering, num::NonZeroU32, ops::Range};

use crate::{Avltriee, AvltrieeAllocator, AvltrieeNode};

pub(crate) type Edge = (Option<NonZeroU32>, Ordering);

pub trait AvltrieeSearch<T, I: ?Sized, A: AvltrieeAllocator<T>>: AsRef<Avltriee<T, I, A>> {
    fn cmp(left: &I, right: &I) -> Ordering;
    fn value(&self, row: NonZeroU32) -> Option<&I>;
    unsafe fn value_unchecked(&self, row: NonZeroU32) -> &I;
    unsafe fn node_value_unchecked(&self, row: NonZeroU32) -> (&AvltrieeNode<T>, &I);

    /// Search row of a value.
    fn row(&self, value: &I) -> Option<NonZeroU32> {
        let edge = self.edge(value);
        (edge.1 == Ordering::Equal).then(|| edge.0).flatten()
    }

    /// Finds the edge of a node from the specified value.
    fn edge(&self, value: &I) -> Edge {
        let triee = self.as_ref();
        let mut row: Option<NonZeroU32> = triee.root();
        let mut ord = Ordering::Equal;
        while let Some(row_inner) = row {
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            ord = Self::cmp(node_value, value);
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

    /// Search >= value.
    fn ge(&self, value: &I) -> Option<NonZeroU32> {
        let triee = self.as_ref();
        let mut row = triee.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            match Self::cmp(node_value, value) {
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
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            match Self::cmp(node_value, value) {
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
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            match Self::cmp(node_value, value) {
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
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            match Self::cmp(node_value, value) {
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
            let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
            match Self::cmp(node_value, start_value) {
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
            if Self::cmp(unsafe { self.value_unchecked(start) }, end_value) != Ordering::Greater {
                row = triee.root();
                let mut end = None;
                while let Some(row_inner) = row {
                    let (node, node_value) = unsafe { self.node_value_unchecked(row_inner) };
                    match Self::cmp(node_value, end_value) {
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
