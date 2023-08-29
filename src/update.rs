mod balance;
mod delete;

use std::cmp::Ordering;

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
    fn delete_before_update(&mut self, row: u32, delete_node: &T);
}

impl<T> AvltrieeHolder<T, T> for Avltriee<T>
where
    T: Ord,
{
    fn cmp(&self, left: &T, right: &T) -> Ordering {
        left.cmp(right)
    }
    fn search_end(&self, input: &T) -> Found {
        self.search_end(|v| v.cmp(input))
    }
    fn value(&mut self, input: T) -> T {
        input
    }
    fn delete_before_update(&mut self, row: u32, _: &T) {
        unsafe {
            self.delete(row);
        }
    }
}

impl<T> Avltriee<T> {
    pub unsafe fn update(&mut self, row: u32, value: T)
    where
        T: Ord + Clone,
    {
        Self::update_holder(self, row, value)
    }

    pub unsafe fn update_holder<I>(holder: &mut dyn AvltrieeHolder<T, I>, row: u32, input: I)
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
            holder.as_mut().update_same(row, found.row);
        } else {
            let value = holder.value(input);
            holder.as_mut().insert_unique(row, value, found);
        }
    }

    pub unsafe fn insert_unique(&mut self, row: u32, value: T, found: Found) {
        *self.offset_mut(row) = AvltrieeNode::new(row, found.row, value);
        if found.row == 0 {
            self.set_root(row);
        } else {
            assert!(
                found.ord != Ordering::Equal,
                "Avltriee.insert_unique : {:?}",
                &found
            );
            let p = self.offset_mut(found.row);
            if found.ord == Ordering::Greater {
                p.left = row;
            } else {
                p.right = row;
            }
            self.balance(row);
        }
    }

    pub(crate) unsafe fn update_same(&mut self, row: u32, same: u32)
    where
        T: Clone,
    {
        let same_node = self.offset_mut(same);
        let node = self.offset_mut(row);

        *node = same_node.clone();

        self.change_row(node, same, row);

        same_node.parent = row;
        node.same = same;
        self.set_parent(node.left, row);
        self.set_parent(node.right, row);

        same_node.left = 0;
        same_node.right = 0;
    }

    fn set_root(&mut self, row: u32) {
        self.node_list.parent = row;
    }

    unsafe fn calc_height(&mut self, row: u32) {
        let node = &mut self.offset_mut(row);
        self.calc_height_node(node);
    }
    unsafe fn calc_height_node(&mut self, node: &mut AvltrieeNode<T>) {
        node.height = std::cmp::max(
            self.offset(node.left).height,
            self.offset(node.right).height,
        ) + 1;
    }

    fn join_intermediate(parent: &mut AvltrieeNode<T>, target_row: u32, child_row: u32) {
        if parent.right == target_row {
            parent.right = child_row;
        } else if parent.left == target_row {
            parent.left = child_row;
        }
    }
    unsafe fn change_row(&mut self, node: &mut AvltrieeNode<T>, target_row: u32, child_row: u32) {
        if node.parent == 0 {
            self.set_root(child_row);
        } else {
            Self::join_intermediate(self.offset_mut(node.parent), target_row, child_row);
        }
    }

    unsafe fn set_parent(&mut self, row: u32, parent: u32) {
        if row != 0 {
            self.offset_mut(row).parent = parent;
        }
    }
}
