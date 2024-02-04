use std::{cmp::Ordering, num::NonZeroU32, ops::Range};

use crate::{Avltriee, AvltrieeAllocator, AvltrieeOrd, Found};

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
    /// Finds the edge of a node from the specified value.
    pub fn search(&self, value: &I) -> Found
    where
        Self: AvltrieeOrd<T, I, A>,
    {
        self.search_edge(self, value)
    }

    /// Finds the edge of a node from the specified value with custom ord.
    pub fn search_edge(&self, o: &impl AvltrieeOrd<T, I, A>, value: &I) -> Found {
        let mut row: Option<NonZeroU32> = self.root();
        let mut ord = Ordering::Equal;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            ord = o.cmp(node, value);
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
        Found { row, ord }
    }

    /// Search >= value with custom ord.
    pub fn search_ge(&self, o: &impl AvltrieeOrd<T, I, A>, value: &I) -> Option<NonZeroU32> {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match o.cmp(node, value) {
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

    /// Search <= value with custom ord.
    pub fn search_le(&self, o: &impl AvltrieeOrd<T, I, A>, value: &I) -> Option<NonZeroU32> {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match o.cmp(node, value) {
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

    /// Search > value with custom ord.
    pub fn search_gt(&self, o: &impl AvltrieeOrd<T, I, A>, value: &I) -> Option<NonZeroU32> {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match o.cmp(node, value) {
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
                        return self.min(node.right);
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { self.get_unchecked(parent).left } == row {
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

    /// Search < value with custom ord.
    pub fn search_lt(&self, o: &impl AvltrieeOrd<T, I, A>, value: &I) -> Option<NonZeroU32> {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match o.cmp(node, value) {
                Ordering::Greater => {
                    if node.left.is_some() {
                        row = node.left;
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    if node.left.is_some() {
                        return self.max(node.left);
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { self.get_unchecked(parent) }.right == row {
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
    pub fn search_range(
        &self,
        o: &impl AvltrieeOrd<T, I, A>,
        start_value: &I,
        end_value: &I,
    ) -> Option<Range<NonZeroU32>> {
        let mut row = self.root();
        let mut start = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match o.cmp(node, start_value) {
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
            if o.cmp(unsafe { self.get_unchecked(start) }, end_value) != Ordering::Greater {
                row = self.root();
                let mut end = None;
                while let Some(row_inner) = row {
                    let node = unsafe { self.get_unchecked(row_inner) };
                    match o.cmp(node, end_value) {
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
