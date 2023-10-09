#[derive(Clone, Debug)]
pub struct AvltrieeNode<T: Copy> {
    pub(super) parent: u32,
    pub(super) left: u32,
    pub(super) right: u32,
    pub(super) same: u32,
    pub(super) height: u8,
    value: T,
}

impl<T: Copy> AvltrieeNode<T> {
    #[inline(always)]
    pub fn new(row: u32, parent: u32, value: T) -> AvltrieeNode<T> {
        AvltrieeNode {
            height: if row == 0 { 0 } else { 1 },
            parent,
            left: 0,
            right: 0,
            same: 0,
            value,
        }
    }
}

impl<T: Copy> std::ops::Deref for AvltrieeNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
