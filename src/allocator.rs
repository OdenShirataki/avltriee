use std::num::NonZeroU32;

use crate::AvltrieeNode;

pub trait AvltrieeAllocator<T> {
    fn as_ptr(&self) -> *const AvltrieeNode<T>;
    fn as_mut_ptr(&mut self) -> *mut AvltrieeNode<T>;

    fn get(&self, row: NonZeroU32) -> Option<&AvltrieeNode<T>>;

    fn resize(&mut self, rows_count: u32);
}

pub struct VecAvltrieeAllocator<T> {
    node_list: Vec<AvltrieeNode<T>>,
}

impl<T: Default + Clone> AvltrieeAllocator<T> for VecAvltrieeAllocator<T> {
    fn as_ptr(&self) -> *const AvltrieeNode<T> {
        self.node_list.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut AvltrieeNode<T> {
        self.node_list.as_mut_ptr()
    }

    fn get(&self, row: NonZeroU32) -> Option<&AvltrieeNode<T>> {
        self.node_list.get(row.get() as usize)
    }

    fn resize(&mut self, rows_count: u32) {
        self.node_list
            .resize(rows_count as usize + 1, Default::default())
    }
}

impl<T: Default> VecAvltrieeAllocator<T> {
    pub fn new() -> Self {
        VecAvltrieeAllocator {
            node_list: vec![Default::default()],
        }
    }
}
