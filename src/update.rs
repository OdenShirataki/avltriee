mod balance;
mod delete;

use std::{cmp::Ordering, num::NonZeroU32};

use super::{Avltriee, AvltrieeNode, Found};

impl<T> AsRef<Avltriee<T>> for Avltriee<T> {
    fn as_ref(&self) -> &Avltriee<T> {
        self
    }
}
impl<T> AsMut<Avltriee<T>> for Avltriee<T> {
    fn as_mut(&mut self) -> &mut Avltriee<T> {
        self
    }
}

pub trait AvltrieeHolder<T, I>
where
    Self: AsRef<Avltriee<T>> + AsMut<Avltriee<T>>,
{
    fn cmp(&self, left: &T, right: &I) -> Ordering;
    fn search_end(&self, input: &I) -> Found;
    fn value(&mut self, input: I) -> T;
    fn delete_before_update(&mut self, row: NonZeroU32, delete_node: &T);
}

impl<T> AvltrieeHolder<T, T> for Avltriee<T>
where
    T: Ord,
{
    #[inline(always)]
    fn cmp(&self, left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }

    #[inline(always)]
    fn search_end(&self, input: &T) -> Found {
        self.search_end(|v| v.cmp(input))
    }

    #[inline(always)]
    fn value(&mut self, input: T) -> T {
        input
    }

    #[inline(always)]
    fn delete_before_update(&mut self, row: NonZeroU32, _: &T) {
        unsafe {
            self.delete(row);
        }
    }
}

impl<T> Avltriee<T> {
    #[inline(always)]
    pub unsafe fn update(&mut self, row: NonZeroU32, value: T)
    where
        T: Ord + Clone,
    {
        Self::update_holder(self, row, value)
    }

    #[inline(always)]
    pub unsafe fn update_holder<I>(holder: &mut dyn AvltrieeHolder<T, I>, row: NonZeroU32, input: I)
    where
        T: Clone,
    {
        if let Some(node) = holder.as_ref().node(row) {
            if holder.cmp(node, &input) == Ordering::Equal {
                return; //update value eq exists value
            }
            holder.delete_before_update(row, node);
        }

        let found = holder.search_end(&input);
        if found.ord == Ordering::Equal && found.row != 0 {
            let same = found.row;
            let t = holder.as_mut();

            let same_node = t.offset_mut(same);
            let node = t.offset_mut(row.get());

            *node = same_node.clone();

            t.change_row(node, NonZeroU32::new_unchecked(same), row);

            same_node.parent = row.get();
            node.same = same;
            t.set_parent(node.left, row.get());
            t.set_parent(node.right, row.get());

            same_node.left = 0;
            same_node.right = 0;
        } else {
            let value = holder.value(input);
            holder.as_mut().insert_unique(row, value, found);
        }
    }

    #[inline(always)]
    pub unsafe fn insert_unique(&mut self, row: NonZeroU32, value: T, found: Found) {
        *self.offset_mut(row.get()) = AvltrieeNode::new(row.get(), found.row, value);
        if found.row == 0 {
            self.set_root(row.get());
        } else {
            let p = self.offset_mut(found.row);
            if found.ord == Ordering::Greater {
                p.left = row.get();
            } else {
                p.right = row.get();
            }
            self.balance(row);
        }
    }

    #[inline(always)]
    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }

    #[inline(always)]
    fn calc_height(&mut self, row: NonZeroU32) {
        let node = unsafe { self.offset_mut(row.get()) };
        self.calc_height_node(node);
    }

    #[inline(always)]
    fn calc_height_node(&self, node: &mut AvltrieeNode<T>) {
        node.height = unsafe {
            std::cmp::max(
                self.offset(node.left).height,
                self.offset(node.right).height,
            )
        } + 1;
    }

    #[inline(always)]
    fn join_intermediate(
        parent: &mut AvltrieeNode<T>,
        target_row: NonZeroU32,
        child_row: NonZeroU32,
    ) {
        let target_row = target_row.get();
        if parent.right == target_row {
            parent.right = child_row.get();
        } else if parent.left == target_row {
            parent.left = child_row.get();
        }
    }

    #[inline(always)]
    fn change_row(
        &mut self,
        node: &mut AvltrieeNode<T>,
        target_row: NonZeroU32,
        child_row: NonZeroU32,
    ) {
        if node.parent == 0 {
            self.set_root(child_row.get());
        } else {
            Self::join_intermediate(
                unsafe { self.offset_mut(node.parent) },
                target_row,
                child_row,
            );
        }
    }

    #[inline(always)]
    fn set_parent(&mut self, row: u32, parent: u32) {
        if row != 0 {
            unsafe { self.offset_mut(row) }.parent = parent;
        }
    }
}
