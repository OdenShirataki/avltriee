#[derive(Clone, Debug, Default)]
pub struct AvltrieeNode<T> {
    pub(super) parent: u32,
    pub(super) left: u32,
    pub(super) right: u32,
    pub(super) same: u32,
    pub(super) height: u8,
    value: T,
}

impl<T> AvltrieeNode<T> {
    pub fn new(row: u32, parent: u32, value: T) -> Self {
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

impl<T> std::ops::Deref for AvltrieeNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
