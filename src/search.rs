use std::{cmp::Ordering, num::NonZeroU32, ops::Range};

use crate::{Avltriee, AvltrieeAllocator, Found};

pub trait AvltrieeSearch<T, I: ?Sized, A: AvltrieeAllocator<T>>: AsRef<Avltriee<T, I, A>> {
    fn cmp(&self, left: &T, right: &I) -> Ordering;
    fn convert<'a, 'b: 'a>(&'a self, value: &'b T) -> &I;

    /// Finds the edge of a node from the specified value.
    fn search(&self, value: &I) -> Found
    where
        Self: Sized,
    {
        edge(self, value)
    }

    /// Search row of a value.
    fn row(&self, value: &I) -> Option<NonZeroU32>
    where
        Self: Sized,
    {
        let found = self.search(value);
        (found.ord == Ordering::Equal).then(|| found.row).flatten()
    }

    /// Returns the value of the specified row. Returns None if the row does not exist.
    fn value<'a>(&'a self, row: NonZeroU32) -> Option<&I>
    where
        A: 'a,
        T: 'a,
    {
        self.as_ref().get(row).map(|v| self.convert(&*v))
    }
}

/// Finds the edge of a node from the specified value with custom ord.
pub fn edge<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    value: &I,
) -> Found {
    let triee = s.as_ref();
    let mut row: Option<NonZeroU32> = triee.root();
    let mut ord = Ordering::Equal;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        ord = s.cmp(node, value);
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
pub fn ge<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    value: &I,
) -> Option<NonZeroU32> {
    let triee = s.as_ref();
    let mut row = triee.root();
    let mut keep = None;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        match s.cmp(node, value) {
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
pub fn le<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    value: &I,
) -> Option<NonZeroU32> {
    let triee = s.as_ref();
    let mut row = triee.root();
    let mut keep = None;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        match s.cmp(node, value) {
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
pub fn gt<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    value: &I,
) -> Option<NonZeroU32> {
    let triee = s.as_ref();
    let mut row = triee.root();
    let mut keep = None;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        match s.cmp(node, value) {
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
                    if unsafe { triee.get_unchecked(parent).left } == row {
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
pub fn lt<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    value: &I,
) -> Option<NonZeroU32> {
    let triee = s.as_ref();
    let mut row = triee.root();
    let mut keep = None;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        match s.cmp(node, value) {
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
                    if unsafe { triee.get_unchecked(parent) }.right == row {
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
pub fn range<T, I: ?Sized, A: AvltrieeAllocator<T>>(
    s: &impl AvltrieeSearch<T, I, A>,
    start_value: &I,
    end_value: &I,
) -> Option<Range<NonZeroU32>> {
    let triee = s.as_ref();
    let mut row = triee.root();
    let mut start = None;
    while let Some(row_inner) = row {
        let node = unsafe { triee.get_unchecked(row_inner) };
        match s.cmp(node, start_value) {
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
        if s.cmp(unsafe { triee.get_unchecked(start) }, end_value) != Ordering::Greater {
            row = triee.root();
            let mut end = None;
            while let Some(row_inner) = row {
                let node = unsafe { triee.get_unchecked(row_inner) };
                match s.cmp(node, end_value) {
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
