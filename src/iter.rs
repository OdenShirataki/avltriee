use std::{cmp::Ordering, ops::Range};

use super::Avltriee;

#[derive(PartialEq)]
enum Order {
    Asc,
    Desc,
}

pub struct AvltrieeIter<'a, T> {
    now: u32,
    end_row: u32,
    same_branch: u32,
    triee: &'a Avltriee<T>,
    next_func: unsafe fn(&Avltriee<T>, u32, u32) -> Option<(u32, u32)>,
}
impl<'a, T> AvltrieeIter<'a, T> {
    fn new(triee: &'a Avltriee<T>, now: u32, end_row: u32, order: Order) -> AvltrieeIter<'a, T> {
        match order {
            Order::Asc => AvltrieeIter {
                now,
                end_row,
                same_branch: 0,
                triee,
                next_func: Avltriee::<T>::next,
            },
            Order::Desc => AvltrieeIter {
                now: end_row,
                end_row: now,
                same_branch: 0,
                triee,
                next_func: Avltriee::<T>::next_desc,
            },
        }
    }
}

impl<'a, T> Iterator for AvltrieeIter<'a, T> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        (self.now != 0).then(|| {
            let c = self.now;
            self.now = unsafe {
                if c == self.end_row {
                    let same = self.triee.offset(c).same;
                    if same != 0 {
                        self.end_row = same;
                    }
                    same
                } else {
                    let next_func = self.next_func;
                    next_func(self.triee, self.now, self.same_branch).map_or(0, |(i, b)| {
                        self.same_branch = b;
                        i
                    })
                }
            };
            c
        })
    }
}

impl<T> Avltriee<T> {
    pub fn iter(&self) -> AvltrieeIter<T> {
        unsafe {
            AvltrieeIter::new(
                &self,
                self.min(self.root()),
                self.max(self.root()),
                Order::Asc,
            )
        }
    }
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        unsafe {
            AvltrieeIter::new(
                &self,
                self.min(self.root()),
                self.max(self.root()),
                Order::Desc,
            )
        }
    }

    pub fn iter_by<'a, F>(&'a self, cmp: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let found = self.search_end(cmp);
        let row = if found.ord == Ordering::Equal {
            found.row
        } else {
            0
        };
        AvltrieeIter::new(&self, row, row, Order::Asc)
    }

    unsafe fn search_ge<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = self.offset(row);
            match compare(node) {
                Ordering::Greater => {
                    if node.left == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.left;
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        keep
    }
    unsafe fn iter_from_inner<'a, F>(&'a self, search: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let now = self.search_ge(search);
        AvltrieeIter::new(
            self,
            now,
            if now == 0 { 0 } else { self.max(self.root()) },
            order,
        )
    }
    pub fn iter_from<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_from_inner(search, Order::Asc) }
    }
    pub fn desc_iter_from<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_from_inner(search, Order::Desc) }
    }

    unsafe fn search_gt<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = self.offset(row);
            match compare(node) {
                Ordering::Greater => {
                    if node.left == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.left;
                }
                Ordering::Equal => {
                    return if node.right != 0 {
                        self.min(node.right)
                    } else {
                        if self.offset(node.parent).left == row {
                            node.parent
                        } else {
                            keep
                        }
                    };
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        keep
    }
    unsafe fn iter_over_inner<'a, F>(&'a self, search: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let now = self.search_gt(search);
        AvltrieeIter::new(
            self,
            now,
            if now == 0 { 0 } else { self.max(self.root()) },
            order,
        )
    }
    pub fn iter_over<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_over_inner(search, Order::Asc) }
    }
    pub fn desc_iter_over<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_over_inner(search, Order::Desc) }
    }

    unsafe fn search_le<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = self.offset(row);
            match compare(node) {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    return row;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.right;
                }
            }
        }
        keep
    }
    unsafe fn iter_to_inner<'a, F>(&'a self, search_from: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let end_row = self.search_le(search_from);
        AvltrieeIter::new(
            self,
            if end_row == 0 {
                0
            } else {
                self.min(self.root())
            },
            end_row,
            order,
        )
    }
    pub fn iter_to<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_to_inner(search_from, Order::Asc) }
    }
    pub fn desc_iter_to<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_to_inner(search_from, Order::Desc) }
    }

    unsafe fn search_lt<F>(&self, compare: F) -> u32
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = 0;
        while row != 0 {
            let node = self.offset(row);
            match compare(node) {
                Ordering::Greater => {
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    return if node.left != 0 {
                        self.max(node.left)
                    } else {
                        if self.offset(node.parent).right == row {
                            node.parent
                        } else {
                            keep
                        }
                    };
                }
                Ordering::Less => {
                    if node.right == 0 {
                        return row;
                    }
                    keep = row;
                    row = node.right;
                }
            }
        }
        keep
    }
    unsafe fn iter_under_inner<'a, F>(&'a self, search_from: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let end_row = self.search_lt(search_from);
        AvltrieeIter::new(
            self,
            if end_row == 0 {
                0
            } else {
                self.min(self.root())
            },
            end_row,
            order,
        )
    }
    pub fn iter_under<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_under_inner(search_from, Order::Asc) }
    }
    pub fn desc_iter_under<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_under_inner(search_from, Order::Desc) }
    }

    unsafe fn search_range<S, E>(&self, compare_ge: S, compare_le: E) -> Option<Range<u32>>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut start = 0;
        while row != 0 {
            let node = self.offset(row);
            match compare_ge(node) {
                Ordering::Greater => {
                    start = row;
                    if node.left == 0 {
                        break;
                    }
                    row = node.left;
                }
                Ordering::Equal => {
                    start = row;
                    break;
                }
                Ordering::Less => {
                    if node.right == 0 {
                        break;
                    }
                    row = node.right;
                }
            }
        }
        (start != 0 && compare_le(self.offset(start)) != Ordering::Greater)
            .then(|| {
                row = self.root();
                let mut end = 0;
                while row != 0 {
                    let node = self.offset(row);
                    match compare_le(node) {
                        Ordering::Greater => {
                            if node.left == 0 {
                                break;
                            }
                            row = node.left;
                        }
                        Ordering::Equal => {
                            end = row;
                            break;
                        }
                        Ordering::Less => {
                            end = row;
                            if node.right == 0 {
                                break;
                            }
                            row = node.right;
                        }
                    }
                }
                (end != 0).then(|| Range { start, end })
            })
            .and_then(|v| v)
    }
    unsafe fn iter_range_inner<'a, S, E>(
        &'a self,
        start: S,
        end: E,
        order: Order,
    ) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        if let Some(range) = self.search_range(start, end) {
            AvltrieeIter::new(self, range.start, range.end, order)
        } else {
            AvltrieeIter::new(self, 0, 0, order)
        }
    }
    pub fn iter_range<'a, S, E>(&'a self, start: S, end: E) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_range_inner(start, end, Order::Asc) }
    }
    pub fn desc_iter_range<'a, S, E>(&'a self, start: S, end: E) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        unsafe { self.iter_range_inner(start, end, Order::Desc) }
    }

    unsafe fn next(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let mut current = c;
        let mut node = self.offset(current);
        if node.same != 0 {
            Some((node.same, if same_branch == 0 { c } else { same_branch }))
        } else {
            if same_branch != 0 {
                current = same_branch;
                node = self.offset(same_branch);
            }
            let parent = node.parent;
            if node.right != 0 {
                Some((self.min(node.right), 0))
            } else {
                (parent != 0)
                    .then(|| {
                        if self.offset(parent).left == current {
                            Some((parent, 0))
                        } else {
                            self.retroactive(parent).map(|i| (i, 0))
                        }
                    })
                    .and_then(|v| v)
            }
        }
    }
    unsafe fn retroactive(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        if self.offset(parent).right == c {
            self.retroactive(parent).filter(|p| *p != c)
        } else {
            Some(parent)
        }
    }

    unsafe fn next_desc(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let mut current = c;
        let mut node = self.offset(current);
        if node.same != 0 {
            Some((node.same, if same_branch == 0 { c } else { same_branch }))
        } else {
            if same_branch != 0 {
                current = same_branch;
                node = self.offset(same_branch);
            }
            let parent = node.parent;
            if node.left != 0 {
                Some((self.max(node.left), 0))
            } else {
                (parent != 0)
                    .then(|| {
                        if self.offset(parent).right == current {
                            Some((parent, 0))
                        } else {
                            self.retroactive_desc(parent).map(|i| (i, 0))
                        }
                    })
                    .and_then(|v| v)
            }
        }
    }
    unsafe fn retroactive_desc(&self, c: u32) -> Option<u32> {
        let parent = self.offset(c).parent;
        if self.offset(parent).left == c {
            self.retroactive_desc(parent).filter(|p| *p != c)
        } else {
            Some(parent)
        }
    }
}
