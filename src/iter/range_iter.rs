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
    pub fn new_with_value(
        triee: &'a Avltriee<T>,
        value_min: &T,
        value_max: &'a T,
    ) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let (_, min_row) = triee.search(value_min);
        let (_, max_row) = triee.search(value_max);
        AvltrieeRangeIter {
            now: min_row,
            end_row: max_row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    pub fn new_with_value_max(triee: &'a Avltriee<T>, value_max: &'a T) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let (_, max_row) = triee.search(value_max);
        AvltrieeRangeIter {
            now: unsafe { triee.min(triee.root() as u32) },
            end_row: max_row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    pub fn new(triee: &'a Avltriee<T>, now: u32, end_row: u32) -> AvltrieeRangeIter<'a, T> {
        AvltrieeRangeIter {
            now,
            end_row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
}
