use std::num::NonZeroU32;

#[derive(Clone, Debug, Default)]
pub struct AvltrieeNode<T> {
    pub(super) parent: Option<NonZeroU32>,
    pub(super) left: u32,
    pub(super) right: u32,
    pub(super) same: Option<NonZeroU32>,
    pub(super) height: u8,
    value: T,
}

impl<T> AvltrieeNode<T> {
    pub fn new(row: Option<NonZeroU32>, parent: Option<NonZeroU32>, value: T) -> Self {
        AvltrieeNode {
            height: if row.is_none() { 0 } else { 1 },
            parent,
            left: 0,
            right: 0,
            same: None,
            value,
        }
    }

    pub(crate) fn changeling(&mut self, current_child: NonZeroU32, new_child: NonZeroU32) {
        let current_child = current_child.get();
        if self.right == current_child {
            self.right = new_child.get();
        } else if self.left == current_child {
            self.left = new_child.get();
        }
    }

    pub(crate) fn same_clone(&mut self, self_row: NonZeroU32, new_row: NonZeroU32) -> Self
    where
        T: Clone,
    {
        let cloned = AvltrieeNode {
            height: self.height,
            parent: self.parent,
            left: self.left,
            right: self.right,
            same: Some(self_row),
            value: self.value.clone(),
        };
        self.left = 0;
        self.right = 0;
        self.parent = Some(new_row);

        cloned
    }
}

impl<T> std::ops::Deref for AvltrieeNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
