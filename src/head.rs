use crate::{Avltriee, AvltrieeNode};

pub(crate) struct AvltrieeHead {
    root: u32,
    max_rows: u32,
}

impl<T> Avltriee<T> {
    pub(crate) fn head(&self) -> &AvltrieeHead {
        unsafe { &*(self.node_list.as_ref() as *const AvltrieeNode<T> as *const AvltrieeHead) }
    }

    pub(crate) fn head_mut(&mut self) -> &mut AvltrieeHead {
        unsafe { &mut *(self.node_list.as_mut() as *mut AvltrieeNode<T> as *mut AvltrieeHead) }
    }

    pub(crate) fn set_root(&mut self, row: u32) {
        self.head_mut().root = row;
    }

    pub(crate) fn root(&self) -> u32 {
        self.head().root
    }

    pub(crate) fn set_max_rows(&mut self, rows: u32) {
        self.head_mut().max_rows = rows;
    }

    pub fn update_max_rows(&mut self, rows: u32) {
        if self.max_rows() < rows {
            self.set_max_rows(rows);
        }
    }

    pub fn max_rows(&self) -> u32 {
        self.head().max_rows
    }
}
