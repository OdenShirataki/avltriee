use crate::Avltriee;

use super::AvlTrieeIterResult;

pub struct AvltrieeIter<'a, T> {
    now: u32,
    same_branch: u32,
    local_index: isize,
    triee: &'a Avltriee<T>,
    next_func: unsafe fn(&Avltriee<T>, u32, u32) -> Option<(u32, u32)>,
}

impl<'a, T: Clone + Default> AvltrieeIter<'a, T> {
    pub fn new(triee: &'a Avltriee<T>, order: super::Order) -> AvltrieeIter<'a, T> {
        match order {
            super::Order::Asc => AvltrieeIter {
                now: unsafe { triee.min(triee.root() as u32) },
                same_branch: 0,
                local_index: 0,
                triee,
                next_func: Avltriee::<T>::next,
            },
            super::Order::Desc => AvltrieeIter {
                now: unsafe { triee.max(triee.root() as u32) },
                same_branch: 0,
                local_index: 0,
                triee,
                next_func: Avltriee::<T>::next_desc,
            },
        }
    }
    pub fn begin_at(
        triee: &'a Avltriee<T>,
        begin: u32,
        order: super::Order,
    ) -> AvltrieeIter<'a, T> {
        AvltrieeIter {
            now: begin,
            same_branch: 0,
            local_index: 0,
            triee,
            next_func: match order {
                super::Order::Asc => Avltriee::<T>::next,
                super::Order::Desc => Avltriee::<T>::next_desc,
            },
        }
    }
}

impl<'a, T: Clone + Default> Iterator for AvltrieeIter<'a, T> {
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
