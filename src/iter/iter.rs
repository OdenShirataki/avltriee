use crate::Avltriee;

use super::AvlTrieeIterResult;
use super::Order;

pub struct AvltrieeIter<'a, T> {
    now: u32,
    same_branch: u32,
    local_index: isize,
    triee: &'a Avltriee<T>,
    next_func: unsafe fn(&Avltriee<T>, u32, u32) -> Option<(u32, u32)>,
}

impl<'a, T> AvltrieeIter<'a, T> {
    pub(super) fn new(triee: &'a Avltriee<T>, order: Order) -> AvltrieeIter<'a, T> {
        let root = triee.root() as u32;
        let now = unsafe {
            match order {
                Order::Asc => triee.min(root),
                Order::Desc => triee.max(root),
            }
        };
        match order {
            Order::Asc => AvltrieeIter {
                now,
                same_branch: 0,
                local_index: 0,
                triee,
                next_func: Avltriee::<T>::next,
            },
            Order::Desc => AvltrieeIter {
                now,
                same_branch: 0,
                local_index: 0,
                triee,
                next_func: Avltriee::<T>::next_desc,
            },
        }
    }
}

impl<'a, T> Iterator for AvltrieeIter<'a, T> {
    type Item = AvlTrieeIterResult<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now == 0 {
            None
        } else {
            self.local_index += 1;
            let c = self.now;
            match unsafe {
                let next_func = self.next_func;
                next_func(self.triee, self.now, self.same_branch)
            } {
                Some((i, b)) => {
                    self.now = i;
                    self.same_branch = b;
                }
                _ => {
                    self.now = 0;
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
