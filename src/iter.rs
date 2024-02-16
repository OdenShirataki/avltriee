use std::{cmp::Ordering, num::NonZeroU32};

use crate::{search::AvltrieeSearch, AvltrieeAllocator};

use super::Avltriee;

#[derive(PartialEq)]
enum Order {
    Asc,
    Desc,
}

pub struct AvltrieeIter<'a, T, I: ?Sized, A> {
    now: Option<NonZeroU32>,
    end_row: Option<NonZeroU32>,
    same_branch: Option<NonZeroU32>,
    triee: &'a Avltriee<T, I, A>,
    next_func: fn(
        &Avltriee<T, I, A>,
        NonZeroU32,
        Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)>,
}

impl<'a, T, I: ?Sized, A: AvltrieeAllocator<T>> AvltrieeIter<'a, T, I, A> {
    fn new(
        triee: &'a Avltriee<T, I, A>,
        now: Option<NonZeroU32>,
        end_row: Option<NonZeroU32>,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        match order {
            Order::Asc => AvltrieeIter {
                now,
                end_row,
                same_branch: None,
                triee,
                next_func: Avltriee::<T, I, A>::next,
            },
            Order::Desc => AvltrieeIter {
                now: end_row,
                end_row: now,
                same_branch: None,
                triee,
                next_func: Avltriee::<T, I, A>::next_desc,
            },
        }
    }

    /// Generates an iterator of nodes with the same value as the specified value.
    pub fn by<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        let edge = s.edge(value);
        let row = if edge.1 == Ordering::Equal {
            edge.0
        } else {
            None
        };
        AvltrieeIter::new(triee, row, row, Order::Asc)
    }

    /// Generates an iterator that is greater than or equal to the specified value.
    pub fn from_asc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::from_inner(s, value, Order::Asc)
    }

    /// Generates a descending iterator with greater than or equal to the specified value.
    pub fn from_desc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::from_inner(s, value, Order::Desc)
    }

    fn from_inner<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        value: &I,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        let now = s.ge(value);
        AvltrieeIter::new(triee, now, now.and_then(|_| triee.max(triee.root())), order)
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value.
    pub fn to_asc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::to_inner(s, value, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value. Iterates in descending order.
    pub fn to_desc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::to_inner(s, value, Order::Desc)
    }

    fn to_inner<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        value: &I,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        let end_row = s.le(value);
        AvltrieeIter::new(
            triee,
            end_row.and_then(|_| triee.min(triee.root())),
            end_row,
            order,
        )
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value.
    pub fn over_asc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::over_inner(s, value, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value. Iterates in descending order.
    pub fn over_desc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::over_inner(s, value, Order::Desc)
    }

    fn over_inner<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        value: &I,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        let now = s.gt(value);
        AvltrieeIter::new(triee, now, now.and_then(|_| triee.max(triee.root())), order)
    }

    /// Generates an iterator of nodes with values ​​less than the specified value.
    pub fn under_asc<S: AvltrieeSearch<T, I, A>>(s: &'a S, value: &I) -> AvltrieeIter<'a, T, I, A> {
        Self::under_inner(s, value, Order::Asc)
    }

    /// Generates an iterator of nodes with values ​​less than the specified value. Iterates in descending order.
    pub fn under_desc<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        value: &I,
    ) -> AvltrieeIter<'a, T, I, A> {
        Self::under_inner(s, value, Order::Desc)
    }

    fn under_inner<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        value: &I,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        let end_row = s.lt(value);
        AvltrieeIter::new(
            triee,
            end_row.and_then(|_| triee.min(triee.root())),
            end_row,
            order,
        )
    }

    /// Generates an iterator of nodes with the specified range of values.
    pub fn range_asc<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        start: &I,
        end: &I,
    ) -> AvltrieeIter<'a, T, I, A> {
        Self::range_inner(s, start, end, Order::Asc)
    }

    /// Generates an iterator of nodes with the specified range of values. Iterates in descending order.
    pub fn range_desc<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        start: &I,
        end: &I,
    ) -> AvltrieeIter<'a, T, I, A> {
        Self::range_inner(s, start, end, Order::Desc)
    }

    fn range_inner<S: AvltrieeSearch<T, I, A>>(
        s: &'a S,
        start: &I,
        end: &I,
        order: Order,
    ) -> AvltrieeIter<'a, T, I, A> {
        let triee = s.as_ref();
        if let Some(range) = s.range(start, end) {
            AvltrieeIter::new(triee, Some(range.start), Some(range.end), order)
        } else {
            AvltrieeIter::new(triee, None, None, order)
        }
    }
}

impl<'a, T, I: ?Sized, A: AvltrieeAllocator<T>> Iterator for AvltrieeIter<'a, T, I, A> {
    type Item = NonZeroU32;

    fn next(&mut self) -> Option<Self::Item> {
        self.now.map(|c| {
            self.now = if Some(c) == self.end_row {
                let same = unsafe { self.triee.node_unchecked(c) }.same;
                if same.is_some() {
                    self.end_row = same;
                }
                same
            } else {
                let next_func = self.next_func;
                next_func(self.triee, c, self.same_branch).map(|(i, b)| {
                    self.same_branch = b;
                    i
                })
            };
            c
        })
    }
}

impl<T, I: ?Sized, A: AvltrieeAllocator<T>> Avltriee<T, I, A> {
    /// Generate an iterator.
    pub fn iter(&self) -> AvltrieeIter<T, I, A> {
        AvltrieeIter::new(
            &self,
            self.min(self.root()),
            self.max(self.root()),
            Order::Asc,
        )
    }

    /// Generate an iterator. Iterates in descending order.
    pub fn desc_iter(&self) -> AvltrieeIter<T, I, A> {
        AvltrieeIter::new(
            &self,
            self.min(self.root()),
            self.max(self.root()),
            Order::Desc,
        )
    }

    /// Generates an iterator of nodes with the same value as the specified value.
    pub fn iter_by<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::by(self, value)
    }

    /// Generates an iterator with values ​​starting from the specified value.
    pub fn iter_from<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::from_asc(self, value)
    }

    /// Generates an iterator with values ​​starting from the specified value. Iterates in descending order.
    pub fn desc_iter_from<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::from_desc(self, value)
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value.
    pub fn iter_to<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::to_asc(self, value)
    }

    /// Generates an iterator of nodes with values ​​less than or equal to the specified value. Iterates in descending order.
    pub fn desc_iter_to<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::to_desc(self, value)
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value.
    pub fn iter_over<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::over_asc(self, value)
    }

    /// Generates an iterator of nodes with values ​​greater than the specified value. Iterates in descending order.
    pub fn desc_iter_over<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::over_desc(self, value)
    }

    /// Generates an iterator of nodes with values ​​less than the specified value.
    pub fn iter_under<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::under_asc(self, value)
    }

    /// Generates an iterator of nodes with values ​​less than the specified value. Iterates in descending order.
    pub fn desc_iter_under<'a>(&'a self, value: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::under_desc(self, value)
    }

    /// Generates an iterator of nodes with the specified range of values.
    pub fn iter_range<'a>(&'a self, start: &I, end: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::range_asc(self, start, end)
    }

    /// Generates an iterator of nodes with the specified range of values. Iterates in descending order.
    pub fn desc_iter_range<'a>(&'a self, start: &I, end: &I) -> AvltrieeIter<T, I, A>
    where
        Self: AvltrieeSearch<T, I, A>,
    {
        AvltrieeIter::range_desc(self, start, end)
    }

    fn next(
        &self,
        c: NonZeroU32,
        same_branch: Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)> {
        let mut node = unsafe { self.node_unchecked(c) };
        if let Some(same) = node.same {
            Some((
                same,
                if same_branch.is_some() {
                    same_branch
                } else {
                    Some(c)
                },
            ))
        } else {
            let current = if let Some(same_branch) = same_branch {
                node = unsafe { self.node_unchecked(same_branch) };
                same_branch
            } else {
                c
            };
            if node.right.is_some() {
                Some((self.min(node.right).unwrap(), None))
            } else {
                node.parent.and_then(|parent| {
                    if unsafe { self.node_unchecked(parent) }.left == Some(current) {
                        Some((parent, None))
                    } else {
                        self.retroactive(parent).map(|i| (i, None))
                    }
                })
            }
        }
    }

    fn retroactive(&self, c: NonZeroU32) -> Option<NonZeroU32> {
        unsafe { self.node_unchecked(c) }.parent.and_then(|parent| {
            if unsafe { self.node_unchecked(parent) }.right == Some(c) {
                self.retroactive(parent).filter(|p| p.get() != c.get())
            } else {
                Some(parent)
            }
        })
    }

    fn next_desc(
        &self,
        c: NonZeroU32,
        same_branch: Option<NonZeroU32>,
    ) -> Option<(NonZeroU32, Option<NonZeroU32>)> {
        let mut node = unsafe { self.node_unchecked(c) };
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
                node = unsafe { self.node_unchecked(current) };
            }
            if node.left.is_some() {
                Some((self.max(node.left).unwrap(), None))
            } else {
                node.parent.and_then(|parent| {
                    if unsafe { self.node_unchecked(parent) }.right == Some(current) {
                        Some((parent, None))
                    } else {
                        self.retroactive_desc(parent).map(|i| (i, None))
                    }
                })
            }
        }
    }

    fn retroactive_desc(&self, c: NonZeroU32) -> Option<NonZeroU32> {
        unsafe { self.node_unchecked(c) }.parent.and_then(|parent| {
            if unsafe { self.node_unchecked(parent) }.left == Some(c) {
                self.retroactive_desc(parent).filter(|p| *p != c)
            } else {
                Some(parent)
            }
        })
    }
}
