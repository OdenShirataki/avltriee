use super::AvltrieeNode;

mod iter;
pub use iter::AvltrieeIter;

mod range_iter;
pub use range_iter::AvltrieeRangeIter;

pub enum Order {
    Asc,
    Desc,
}

pub struct AvlTrieeIterResult<'a, T> {
    index: isize,
    row: u32,
    node: &'a AvltrieeNode<T>,
}
impl<'a, T: Clone + Default> AvlTrieeIterResult<'a, T> {
    pub fn index(&self) -> isize {
        self.index
    }
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn value(&self) -> &'a T {
        self.node.value()
    }
}
