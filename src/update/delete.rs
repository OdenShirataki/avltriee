use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    fn delete_same(&mut self, delete_node: &AvltrieeNode<T>) {
        let new_node = unsafe { self.offset_mut(delete_node.same) };

        new_node.parent = delete_node.parent;
        new_node.height = delete_node.height;

        new_node.left = delete_node.left;
        self.set_parent(new_node.left, delete_node.same);

        new_node.right = delete_node.right;
        self.set_parent(new_node.right, delete_node.same);
    }

    unsafe fn delete_intermediate(&mut self, delete_node: &mut AvltrieeNode<T>) -> (u32, u32) {
        let left_max_row = self.max(delete_node.left);
        let left_max = self.offset_mut(left_max_row);

        left_max.right = delete_node.right;
        self.offset_mut(left_max.right).parent = left_max_row;

        if delete_node.left == left_max_row {
            left_max.parent = delete_node.parent;
            self.calc_height_node(left_max);
            (left_max_row, left_max_row)
        } else {
            left_max.height = delete_node.height;

            let left_max_parent_row = left_max.parent;
            let left_max_parent = self.offset_mut(left_max_parent_row);

            left_max_parent.right = left_max.left;
            self.set_parent(left_max_parent.right, left_max_parent_row);

            left_max.left = delete_node.left;
            self.offset_mut(left_max.left).parent = left_max_row;

            (left_max_row, left_max_parent_row)
        }
    }
    pub unsafe fn delete(&mut self, target_row: u32) {
        let delete_node = self.offset_mut(target_row);
        if delete_node.height > 0 {
            let row_parent = delete_node.parent;
            if row_parent == 0 {
                if delete_node.same != 0 {
                    self.set_root(delete_node.same);
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    self.set_root(delete_node.right);
                    self.set_parent(delete_node.right, 0);
                } else if delete_node.right == 0 {
                    self.set_root(delete_node.left);
                    self.offset_mut(delete_node.left).parent = 0;
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    self.set_root(new_row);
                    let node = self.offset_mut(new_row);
                    self.calc_height_to_balance(node, balance_row, 0);
                }
            } else {
                let mut parent = self.offset_mut(row_parent);
                if parent.same == target_row {
                    parent.same = delete_node.same;
                    if delete_node.same != 0 {
                        self.delete_same(delete_node);
                    }
                } else if delete_node.same != 0 {
                    Self::join_intermediate(parent, target_row, delete_node.same);
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    Self::join_intermediate(&mut parent, target_row, delete_node.right);
                    self.set_parent(delete_node.right, row_parent);
                    self.balance(row_parent);
                } else if delete_node.right == 0 {
                    Self::join_intermediate(parent, target_row, delete_node.left);
                    self.offset_mut(delete_node.left).parent = row_parent;
                    self.balance(row_parent);
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    Self::join_intermediate(parent, target_row, new_row);
                    let node = self.offset_mut(new_row);
                    node.height = delete_node.height;
                    self.calc_height_to_balance(node, balance_row, row_parent);
                }
            }
            delete_node.height = 0;
        }
    }

    unsafe fn calc_height_to_balance(&mut self, node: &mut AvltrieeNode<T>, row: u32, parent: u32) {
        node.parent = parent;
        if row != 0 {
            self.calc_height(row);
            self.balance(row);
        }
    }
}
