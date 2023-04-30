use std::cmp::Ordering;

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
    pub fn new(triee: &'a Avltriee<T>, now: u32, end_row: u32) -> AvltrieeRangeIter<'a, T> {
        AvltrieeRangeIter {
            now,
            end_row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }

    pub fn new_with_value(triee: &'a Avltriee<T>, value: &'a T) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let row = {
            let (ord, row) = triee.search(value);
            if ord == Ordering::Equal {
                row
            } else {
                0
            }
        };
        AvltrieeRangeIter {
            now: row,
            end_row: row,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    pub fn new_with_value_from_to(
        triee: &'a Avltriee<T>,
        value_min: &'a T,
        value_max: &'a T,
    ) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let max_row = Self::row_under(triee, value_max);
        if max_row == 0 {
            Self::empty(triee)
        } else {
            let now = Self::row_over(triee, value_min);
            if now == 0 {
                Self::empty(triee)
            } else {
                if unsafe {
                    triee.node(now).unwrap().value() > triee.node(max_row).unwrap().value()
                } {
                    Self::empty(triee)
                } else {
                    AvltrieeRangeIter {
                        now,
                        end_row: if now == 0 { 0 } else { max_row },
                        same_branch: 0,
                        local_index: 0,
                        triee,
                    }
                }
            }
        }
    }
    pub fn new_with_value_from(triee: &'a Avltriee<T>, value_min: &'a T) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let now = Self::row_over(triee, value_min);
        AvltrieeRangeIter {
            now,
            end_row: if now == 0 {
                0
            } else {
                unsafe { triee.max(triee.root()) }
            },
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    pub fn new_with_value_to(triee: &'a Avltriee<T>, value_max: &'a T) -> AvltrieeRangeIter<'a, T>
    where
        T: Ord,
    {
        let max_row = Self::row_under(triee, value_max);
        if max_row == 0 {
            Self::empty(triee)
        } else {
            AvltrieeRangeIter {
                now: unsafe { triee.min(triee.root()) },
                end_row: max_row,
                same_branch: 0,
                local_index: 0,
                triee,
            }
        }
    }
    fn empty(triee: &'a Avltriee<T>) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter {
            now: 0,
            end_row: 0,
            same_branch: 0,
            local_index: 0,
            triee,
        }
    }
    fn row_over(triee: &'a Avltriee<T>, value: &'a T) -> u32
    where
        T: Ord,
    {
        let (ord, row) = triee.search(value);
        if ord == Ordering::Greater {
            let node = unsafe { triee.node(row) }.unwrap();
            if node.right != 0 {
                node.right
            } else {
                node.parent
            }
        } else {
            row
        }
    }
    fn row_under(triee: &'a Avltriee<T>, value: &'a T) -> u32
    where
        T: Ord,
    {
        let (ord, row) = triee.search(value);
        if ord != Ordering::Less {
            row
        } else {
            0
        }
    }
}
