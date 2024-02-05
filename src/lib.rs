pub mod search;

mod allocator;
mod default;
mod head;
mod iter;
mod node;
mod ord;
mod update;

use std::{cmp::Ordering, marker::PhantomData, num::NonZeroU32};

use allocator::VecAvltrieeAllocator;

pub use allocator::AvltrieeAllocator;
pub use iter::AvltrieeIter;
pub use node::AvltrieeNode;
pub use ord::AvltrieeOrd;
pub use update::AvltrieeUpdate;

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

pub struct Avltriee<T, I: ?Sized = T, A = VecAvltrieeAllocator<T>> {
    allocator: A,
    _marker: PhantomData<fn(I, T)>,
}

impl<T: Default> Avltriee<T, T, VecAvltrieeAllocator<T>> {
    /// Creates the Avltriee with Default allocator.
    pub fn new() -> Self {
        Self {
            allocator: VecAvltrieeAllocator::new(),
            _marker: PhantomData,
        }
    }
}

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
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
            .and_then(|node| (node.height != 0).then(|| unsafe { self.get_unchecked(row) }))
    }

    pub unsafe fn get_unchecked(&self, row: NonZeroU32) -> &AvltrieeNode<T> {
        &*self.allocator.as_ptr().offset(row.get() as isize)
    }

    unsafe fn get_unchecked_mut(&mut self, row: NonZeroU32) -> &mut AvltrieeNode<T> {
        &mut *self.allocator.as_mut_ptr().offset(row.get() as isize)
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

    fn allocate(&mut self, rows: NonZeroU32) {
        self.allocator.resize(rows.get());
        self.set_rows_count(rows.get());
    }

    fn min(&self, t: Option<NonZeroU32>) -> Option<NonZeroU32> {
        let mut t = t;
        while let Some(t_inner) = t {
            let l = unsafe { self.get_unchecked(t_inner) }.left;
            if l.is_some() {
                t = l;
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
            if r.is_some() {
                t = r;
            } else {
                break;
            }
        }
        t
    }
}
