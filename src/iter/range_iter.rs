use crate::Avltriee;

use super::AvlTrieeIterResult;

pub struct AvltrieeRangeIter<'a, T> {
    now: u32,
    end_row: u32,
    same_branch: u32,
    local_index: isize,
    triee: &'a Avltriee<T>,
}

impl<'a, T> Iterator for AvltrieeRangeIter<'a, T> {
    type Item = AvlTrieeIterResult<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now == 0 {
            None
        } else {
            self.local_index += 1;
            let c = self.now;
            if c == self.end_row {
                let same = unsafe { self.triee.offset(c) }.same;
                if same != 0 {
                    self.end_row = same;
                }
                self.now = same;
            } else {
                match unsafe { self.triee.next(self.now, self.same_branch) } {
                    Some((i, b)) => {
                        self.now = i;
                        self.same_branch = b;
                    }
                    _ => {
                        self.now = 0;
                    }
                }
            }
            Some(AvlTrieeIterResult {
                index: self.local_index,
                row: c,
                node: unsafe { &self.triee.offset(c) },
            })
        }
    }
}

impl<'a, T> AvltrieeRangeIter<'a, T> {
    pub(super) fn new(triee: &'a Avltriee<T>, now: u32, end_row: u32) -> AvltrieeRangeIter<'a, T> {
        AvltrieeRangeIter {
            now,
            end_row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    pub(super) fn by(triee: &'a Avltriee<T>, row: u32) -> AvltrieeRangeIter<'a, T> {
        Self::new(triee, row, row)
    }
    pub(super) fn from(triee: &'a Avltriee<T>, now: u32) -> AvltrieeRangeIter<'a, T> {
        Self::new(
            triee,
            now,
            if now == 0 {
                0
            } else {
                unsafe { triee.max(triee.root()) }
            },
        )
    }
    pub(super) fn to(triee: &'a Avltriee<T>, end_row: u32) -> AvltrieeRangeIter<'a, T> {
        if end_row == 0 {
            Self::empty(triee)
        } else {
            Self::new(triee, unsafe { triee.min(triee.root()) }, end_row)
        }
    }

    pub(super) fn empty(triee: &'a Avltriee<T>) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter {
            now: 0,
            end_row: 0,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
}
