use crate::{Avltriee, AvltrieeNode};

pub(crate) struct AvltrieeHead {
    root: u32,
    capacity: u32,
}

impl<T> Avltriee<T> {
    fn head(&self) -> &AvltrieeHead {
        unsafe { &*(self.node_list.as_ref() as *const AvltrieeNode<T> as *const AvltrieeHead) }
    }

    fn head_mut(&mut self) -> &mut AvltrieeHead {
        unsafe { &mut *(self.node_list.as_mut() as *mut AvltrieeNode<T> as *mut AvltrieeHead) }
    }

    pub(crate) fn set_root(&mut self, row: u32) {
        self.head_mut().root = row;
    }

    pub(crate) fn root(&self) -> u32 {
        self.head().root
    }

    pub(crate) fn set_capacity(&mut self, rows: u32) {
        self.head_mut().capacity = rows;
    }

    pub(crate) fn extend_capacity(&mut self, rows: u32) {
        if self.capacity() < rows {
            self.set_capacity(rows);
        }
    }

    /// Returns capacity.
    pub fn capacity(&self) -> u32 {
        self.head().capacity
    }
}
