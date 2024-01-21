use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeAllocator};

pub(crate) struct AvltrieeHead {
    root: Option<NonZeroU32>,
    rows_count: u32,
}

impl<T, A: AvltrieeAllocator<T>> Avltriee<T, A> {
    fn head(&self) -> &AvltrieeHead {
        unsafe { &*(self.allocator.as_ptr() as *const AvltrieeHead) }
    }

    fn head_mut(&mut self) -> &mut AvltrieeHead {
        unsafe { &mut *(self.allocator.as_mut_ptr() as *mut AvltrieeHead) }
    }

    pub(crate) fn set_root(&mut self, row: Option<NonZeroU32>) {
        self.head_mut().root = row;
    }

    pub(crate) fn root(&self) -> Option<NonZeroU32> {
        self.head().root
    }

    pub(crate) fn set_rows_count(&mut self, len: u32) {
        self.head_mut().rows_count = len;
    }

    /// Return count of rows.
    pub fn rows_count(&self) -> u32 {
        self.head().rows_count
    }
}
