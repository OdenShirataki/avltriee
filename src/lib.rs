mod allocator;
mod head;
mod iter;
mod node;
mod update;

use std::{cmp::Ordering, marker::PhantomData, num::NonZeroU32};

use allocator::DefaultAvltrieeAllocator;

pub use allocator::AvltrieeAllocator;
pub use iter::AvltrieeIter;
pub use node::AvltrieeNode;
pub use update::AvltrieeHolder;

#[derive(Debug)]
pub struct Found {
    row: Option<NonZeroU32>,
    ord: Ordering,
}
impl Found {
    pub fn row(&self) -> Option<NonZeroU32> {
        self.row
    }

    pub fn ord(&self) -> Ordering {
        self.ord
    }
}

pub struct Avltriee<T, A = DefaultAvltrieeAllocator<T>> {
    allocator: A,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Default + 'static> Avltriee<T, DefaultAvltrieeAllocator<T>> {
    /// Creates the Avltriee with Default allocator.
    pub fn new() -> Self {
        Self {
            allocator: DefaultAvltrieeAllocator::new(),
            _marker: PhantomData,
        }
    }
}

impl<T, A: AvltrieeAllocator<T>> Avltriee<T, A> {
    /// Creates the Avltriee with [AvltrieeAllocator].
    pub fn with_allocator(allocator: A) -> Self {
        Self {
            allocator,
            _marker: PhantomData,
        }
    }

    /// Returns the node of the specified row.
    pub fn get(&self, row: NonZeroU32) -> Option<&AvltrieeNode<T>> {
        self.allocator
            .get(row)
            .and_then(|node| (node.height != 0).then_some(unsafe { self.get_unchecked(row) }))
    }

    pub unsafe fn get_unchecked(&self, row: NonZeroU32) -> &AvltrieeNode<T> {
        &*self.allocator.as_ptr().offset(row.get() as isize)
    }

    unsafe fn get_unchecked_mut(&mut self, row: NonZeroU32) -> &mut AvltrieeNode<T> {
        &mut *self.allocator.as_mut_ptr().offset(row.get() as isize)
    }

    /// Finds the edge of a node from the specified value.
    pub fn search<F: Fn(&T) -> Ordering>(&self, cmp: F) -> Found {
        let mut row = self.root();
        let mut ord = Ordering::Equal;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            ord = cmp(node);
            match ord {
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        row = Some(left);
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        row = Some(right);
                    } else {
                        break;
                    }
                }
            }
        }
        Found { row, ord }
    }

    /// Checks whether the specified row is a node with a unique value.
    pub fn is_unique(&self, row: NonZeroU32) -> Option<(bool, &AvltrieeNode<T>)> {
        self.get(row).map(|node| {
            (
                node.same.is_none()
                    && node.parent.is_some_and(|parent| {
                        unsafe { self.get_unchecked(parent) }.same != Some(row)
                    }),
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

    fn min(&self, t: Option<NonZeroU32>) -> Option<NonZeroU32> {
        let mut t = t;
        while let Some(t_inner) = t {
            let l = unsafe { self.get_unchecked(t_inner) }.left;
            if let Some(l) = l {
                t = Some(l);
            } else {
                break;
            }
        }
        t
    }

    fn max(&self, t: Option<NonZeroU32>) -> Option<NonZeroU32> {
        let mut t = t;
        while let Some(t_inner) = t {
            let r = unsafe { self.get_unchecked(t_inner) }.right;
            if let Some(r) = r {
                t = Some(r);
            } else {
                break;
            }
        }
        t
    }
}
