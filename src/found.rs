use std::cmp::Ordering;

pub struct Found {
    pub(super) row: u32,
    pub(super) ord: Ordering,
}
impl Found {
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn ord(&self) -> Ordering {
        self.ord
    }
}
