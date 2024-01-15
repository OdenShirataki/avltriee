use std::{cmp::Ordering, num::NonZeroU32, ops::Range};

use super::Avltriee;

#[derive(PartialEq)]
enum Order {
    Asc,
    Desc,
}

pub struct AvltrieeIter<'a, T> {
    now: Option<NonZeroU32>,
    end_row: Option<NonZeroU32>,
    same_branch: Option<NonZeroU32>,
    triee: &'a Avltriee<T>,
    next_func: fn(
        &Avltriee<T>,
        NonZeroU32,
        Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)>,
}
impl<'a, T> AvltrieeIter<'a, T> {
    fn new(
        triee: &'a Avltriee<T>,
        now: Option<NonZeroU32>,
        end_row: Option<NonZeroU32>,
        order: Order,
    ) -> AvltrieeIter<'a, T> {
        match order {
            Order::Asc => AvltrieeIter {
                now,
                end_row,
                same_branch: None,
                triee,
                next_func: Avltriee::<T>::next,
            },
            Order::Desc => AvltrieeIter {
                now: end_row,
                end_row: now,
                same_branch: None,
                triee,
                next_func: Avltriee::<T>::next_desc,
            },
        }
    }
}

impl<'a, T> Iterator for AvltrieeIter<'a, T> {
    type Item = NonZeroU32;

    fn next(&mut self) -> Option<Self::Item> {
        self.now.map(|c| {
            self.now = if Some(c) == self.end_row {
                let same = unsafe { self.triee.get_unchecked(c) }.same;
                if let Some(same) = same {
                    self.end_row = Some(same);
                    Some(same)
                } else {
                    None
                }
            } else {
                let next_func = self.next_func;
                next_func(self.triee, c, self.same_branch).map_or(None, |(i, b)| {
                    self.same_branch = b;
                    Some(i)
                })
            };
            c
        })
    }
}

impl<T> Avltriee<T> {
    /// Generate an iterator.
    pub fn iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(
            &self,
            self.min(self.root()),
            self.max(self.root()),
            Order::Asc,
        )
    }

    /// Generate an iterator. Iterates in descending order.
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(
            &self,
            self.min(self.root()),
            self.max(self.root()),
            Order::Desc,
        )
    }

    /// Generates an iterator of nodes with the same value as the specified value.
    pub fn iter_by<'a, F>(&'a self, cmp: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let found = self.search(cmp);
        let row = if found.ord == Ordering::Equal {
            found.row
        } else {
            None
        };
        AvltrieeIter::new(&self, row, row, Order::Asc)
    }

    fn search_ge<F>(&self, compare: F) -> Option<NonZeroU32>
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match compare(node) {
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        keep = Some(row_inner);
                        row = Some(left);
                    } else {
                        return Some(row_inner);
                    }
                }
                Ordering::Equal => {
                    return Some(row_inner);
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        row = Some(right);
                    } else {
                        break;
                    }
                }
            }
        }
        keep
    }

    fn iter_from_inner<'a, F>(&'a self, search: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let now = self.search_ge(search);
        AvltrieeIter::new(
            self,
            now,
            if now.is_none() {
                None
            } else {
                self.max(self.root())
            },
            order,
        )
    }

    /// Generates an iterator with values ​​starting from the specified value.
    pub fn iter_from<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_from_inner(search, Order::Asc)
    }

    /// Generates an iterator with values ​​starting from the specified value. Iterates in descending order.
    pub fn desc_iter_from<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_from_inner(search, Order::Desc)
    }

    fn search_gt<F>(&self, compare: F) -> Option<NonZeroU32>
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match compare(node) {
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        keep = Some(row_inner);
                        row = Some(left);
                    } else {
                        return Some(row_inner);
                    }
                }
                Ordering::Equal => {
                    if let Some(right) = node.right {
                        return self.min(Some(right));
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { self.get_unchecked(parent).left } == Some(row_inner) {
                            return Some(parent);
                        }
                    }
                    break;
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        row = Some(right);
                    } else {
                        break;
                    }
                }
            }
        }
        keep
    }

    fn iter_over_inner<'a, F>(&'a self, search: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let now = self.search_gt(search);
        AvltrieeIter::new(
            self,
            now,
            if now.is_none() {
                None
            } else {
                self.max(self.root())
            },
            order,
        )
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value.
    pub fn iter_over<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_over_inner(search, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value. Iterates in descending order.
    pub fn desc_iter_over<'a, F>(&'a self, search: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_over_inner(search, Order::Desc)
    }

    fn search_le<F>(&self, compare: F) -> Option<NonZeroU32>
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match compare(node) {
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        row = Some(left);
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    return Some(row_inner);
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        keep = Some(row_inner);
                        row = Some(right);
                    } else {
                        return Some(row_inner);
                    }
                }
            }
        }
        keep
    }

    fn iter_to_inner<'a, F>(&'a self, search_from: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let end_row = self.search_le(search_from);
        AvltrieeIter::new(
            self,
            if end_row.is_none() {
                None
            } else {
                self.min(self.root())
            },
            end_row,
            order,
        )
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value.
    pub fn iter_to<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_to_inner(search_from, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value. Iterates in descending order.
    pub fn desc_iter_to<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_to_inner(search_from, Order::Desc)
    }

    fn search_lt<F>(&self, compare: F) -> Option<NonZeroU32>
    where
        F: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut keep = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match compare(node) {
                Ordering::Greater => {
                    if let Some(left) = node.left {
                        row = Some(left);
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    if let Some(left) = node.left {
                        return self.max(Some(left));
                    }
                    if let Some(parent) = node.parent {
                        if unsafe { self.get_unchecked(parent) }.right == Some(row_inner) {
                            return Some(parent);
                        }
                    }
                    break;
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        keep = Some(row_inner);
                        row = Some(right);
                    } else {
                        return Some(row_inner);
                    }
                }
            }
        }
        keep
    }

    fn iter_under_inner<'a, F>(&'a self, search_from: F, order: Order) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        let end_row = self.search_lt(search_from);
        AvltrieeIter::new(
            self,
            if end_row.is_none() {
                None
            } else {
                self.min(self.root())
            },
            end_row,
            order,
        )
    }

    /// Generates an iterator of nodes with values ​​less than the specified value.
    pub fn iter_under<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_under_inner(search_from, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​less than the specified value. Iterates in descending order.
    pub fn desc_iter_under<'a, F>(&'a self, search_from: F) -> AvltrieeIter<T>
    where
        F: Fn(&T) -> Ordering,
    {
        self.iter_under_inner(search_from, Order::Desc)
    }

    fn search_range<S, E>(&self, compare_ge: S, compare_le: E) -> Option<Range<NonZeroU32>>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        let mut row = self.root();
        let mut start = None;
        while let Some(row_inner) = row {
            let node = unsafe { self.get_unchecked(row_inner) };
            match compare_ge(node) {
                Ordering::Greater => {
                    start = Some(row_inner);
                    if let Some(left) = node.left {
                        row = Some(left);
                    } else {
                        break;
                    }
                }
                Ordering::Equal => {
                    start = Some(row_inner);
                    break;
                }
                Ordering::Less => {
                    if let Some(right) = node.right {
                        row = Some(right);
                    } else {
                        break;
                    }
                }
            }
        }
        if let Some(start) = start {
            if compare_le(unsafe { self.get_unchecked(start) }) != Ordering::Greater {
                row = self.root();
                let mut end = None;
                while let Some(row_inner) = row {
                    let node = unsafe { self.get_unchecked(row_inner) };
                    match compare_le(node) {
                        Ordering::Greater => {
                            if let Some(left) = node.left {
                                row = Some(left);
                            } else {
                                break;
                            }
                        }
                        Ordering::Equal => {
                            end = Some(row_inner);
                            break;
                        }
                        Ordering::Less => {
                            end = Some(row_inner);
                            if let Some(right) = node.right {
                                row = Some(right);
                            } else {
                                break;
                            }
                        }
                    }
                }
                if let Some(end) = end {
                    return Some(Range { start, end });
                }
            }
        }
        None
    }

    fn iter_range_inner<'a, S, E>(&'a self, start: S, end: E, order: Order) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        if let Some(range) = self.search_range(start, end) {
            AvltrieeIter::new(
                self,
                Some(unsafe { NonZeroU32::new_unchecked(range.start.get()) }),
                Some(unsafe { NonZeroU32::new_unchecked(range.end.get()) }),
                order,
            )
        } else {
            AvltrieeIter::new(self, None, None, order)
        }
    }

    /// Generates an iterator of nodes with the specified range of values.
    pub fn iter_range<'a, S, E>(&'a self, start: S, end: E) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        self.iter_range_inner(start, end, Order::Asc)
    }

    /// Generates an iterator of nodes with the specified range of values. Iterates in descending order.
    pub fn desc_iter_range<'a, S, E>(&'a self, start: S, end: E) -> AvltrieeIter<T>
    where
        S: Fn(&T) -> Ordering,
        E: Fn(&T) -> Ordering,
    {
        self.iter_range_inner(start, end, Order::Desc)
    }

    fn next(
        &self,
        c: NonZeroU32,
        same_branch: Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)> {
        let mut node = unsafe { self.get_unchecked(c) };
        if let Some(same) = node.same {
            Some((
                same,
                Some(if let Some(same_branch) = same_branch {
                    same_branch
                } else {
                    c
                }),
            ))
        } else {
            let mut current = c;
            if let Some(same_branch) = same_branch {
                current = same_branch;
                node = unsafe { self.get_unchecked(current) };
            }
            if let Some(right) = node.right {
                Some((self.min(Some(right)).unwrap(), None))
            } else {
                if let Some(parent) = node.parent {
                    if unsafe { self.get_unchecked(parent) }.left == Some(current) {
                        Some((parent, None))
                    } else {
                        self.retroactive(parent).map(|i| (i, None))
                    }
                } else {
                    None
                }
            }
        }
    }

    fn retroactive(&self, c: NonZeroU32) -> Option<NonZeroU32> {
        if let Some(parent) = unsafe { self.get_unchecked(c) }.parent {
            if unsafe { self.get_unchecked(parent) }.right == Some(c) {
                self.retroactive(parent).filter(|p| p.get() != c.get())
            } else {
                Some(parent)
            }
        } else {
            None
        }
    }

    fn next_desc(
        &self,
        c: NonZeroU32,
        same_branch: Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)> {
        let mut node = unsafe { self.get_unchecked(c) };
        if let Some(same) = node.same {
            Some((
                same,
                Some(if let Some(same_branch) = same_branch {
                    same_branch
                } else {
                    c
                }),
            ))
        } else {
            let mut current = c;
            if let Some(same_branch) = same_branch {
                current = same_branch;
                node = unsafe { self.get_unchecked(current) };
            }
            if let Some(left) = node.left {
                Some((self.max(Some(left)).unwrap(), None))
            } else {
                if let Some(parent) = node.parent {
                    if unsafe { self.get_unchecked(parent) }.right == Some(current) {
                        Some((parent, None))
                    } else {
                        self.retroactive_desc(parent).map(|i| (i, None))
                    }
                } else {
                    None
                }
            }
        }
    }

    fn retroactive_desc(&self, c: NonZeroU32) -> Option<NonZeroU32> {
        if let Some(parent) = unsafe { self.get_unchecked(c) }.parent {
            if unsafe { self.get_unchecked(parent) }.left == Some(c) {
                self.retroactive_desc(parent).filter(|p| *p != c)
            } else {
                Some(parent)
            }
        } else {
            None
        }
    }
}
