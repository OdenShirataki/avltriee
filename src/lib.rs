mod allocator;
mod head;
mod iter;
mod node;
mod update;

pub use allocator::AvltrieeAllocator;
use allocator::DefaultAvltrieeAllocator;
pub use iter::AvltrieeIter;
pub use node::AvltrieeNode;
pub use update::AvltrieeHolder;

use std::{cmp::Ordering, num::NonZeroU32};

#[derive(Debug)]
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
    allocator: Box<dyn AvltrieeAllocator<T>>,
}

impl<T> Avltriee<T> {
    /// Creates the Avltriee<T>.
    pub fn new() -> Self
    where
        T: Default + 'static,
    {
        Self {
            allocator: Box::new(DefaultAvltrieeAllocator::new()),
        }
    }

    /// Creates the Avltriee<T> with [AvltrieeAllocator].
    pub fn with_allocator(allocator: Box<dyn AvltrieeAllocator<T>>) -> Self {
        Self { allocator }
    }

    /// Returns the node of the specified row.
    pub fn get(&self, row: NonZeroU32) -> Option<&AvltrieeNode<T>> {
        if let Some(node) = self.allocator.get(row) {
            if node.height > 0 {
                Some(unsafe { self.get_unchecked(row) })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, row: NonZeroU32) -> &AvltrieeNode<T> {
        &*self.allocator.as_ptr().offset(row.get() as isize)
    }

    unsafe fn get_unchecked_mut<'a>(&mut self, row: NonZeroU32) -> &'a mut AvltrieeNode<T> {
        &mut *self.allocator.as_mut_ptr().offset(row.get() as isize)
    }

    /// Finds the edge of a node from the specified value.
    pub fn search<F>(&self, cmp: F) -> Found
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut ord = Ordering::Equal;
        while row != 0 {
            let node = unsafe { self.get_unchecked(NonZeroU32::new_unchecked(row)) };
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

    /// Checks whether the specified row is a node with a unique value.
    pub fn is_unique(&self, row: NonZeroU32) -> Option<(bool, &AvltrieeNode<T>)> {
        self.get(row).map(|node| {
            (
                node.same == 0
                    && (node.parent == 0
                        || unsafe { self.get_unchecked(NonZeroU32::new_unchecked(node.parent)) }
                            .same
                            != row.get()),
                node,
            )
        })
    }

    fn allocate(&mut self, rows: NonZeroU32)
    where
        T: Clone + Default,
    {
        self.allocator.resize(rows.get());
        self.set_rows_count(rows.get());
    }

    fn min(&self, t: u32) -> u32 {
        let mut t = t;
        while t != 0 {
            let l = unsafe { self.get_unchecked(NonZeroU32::new_unchecked(t)) }.left;
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
            let r = unsafe { self.get_unchecked(NonZeroU32::new_unchecked(t)) }.right;
            if r == 0 {
                break;
            }
            t = r;
        }
        t
    }
}
