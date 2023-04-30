#[derive(Clone, Debug)]
pub struct AvltrieeNode<T> {
    pub(super) parent: u32,
    pub(super) left: u32,
    pub(super) right: u32,
    pub(super) same: u32,
    pub(super) height: u8,
    pub(super) value: T,
}
impl<T> AvltrieeNode<T> {
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

    pub(super) fn reset(&mut self) {
        self.height = 0;
        self.parent = 0;
        self.left = 0;
        self.right = 0;
        self.same = 0;
    }
}
