use std::num::{NonZeroU32, NonZeroU8};

#[derive(Clone, Debug, Default)]
pub struct AvltrieeNode<T> {
    pub(super) parent: Option<NonZeroU32>,
    pub(super) left: Option<NonZeroU32>,
    pub(super) right: Option<NonZeroU32>,
    pub(super) same: Option<NonZeroU32>,
    pub(super) height: Option<NonZeroU8>,
    value: T,
}

impl<T> AvltrieeNode<T> {
    pub fn new(row: Option<NonZeroU32>, parent: Option<NonZeroU32>, value: T) -> Self {
        AvltrieeNode {
            height: NonZeroU8::new(if row.is_none() { 0 } else { 1 }),
            parent,
            left: None,
            right: None,
            same: None,
            value,
        }
    }

    pub(crate) fn changeling(&mut self, current_child: NonZeroU32, new_child: Option<NonZeroU32>) {
        if self.right == Some(current_child) {
            self.right = new_child;
        } else if self.left == Some(current_child) {
            self.left = new_child;
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
        self.left = None;
        self.right = None;
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
