use std::{
    cmp::{Ord, Ordering},
    mem::ManuallyDrop,
};

mod iter;
pub use iter::{AvltrieeIter, AvltrieeRangeIter};

#[derive(Clone)]
pub struct AvltrieeNode<T> {
    parent: u32,
    left: u32,
    right: u32,
    same: u32,
    height: u8,
    value: T,
}
impl<T: Clone + Default> AvltrieeNode<T> {
    pub fn new(row: u32, parent: u32, value: T) -> AvltrieeNode<T> {
        AvltrieeNode {
            height: if row == 0 { 0 } else { 1 },
            parent,
            left: 0,
            right: 0,
            same: 0,
            value: value.clone(),
        }
    }
    pub fn reset(&mut self) {
        self.height = 0;
        self.parent = 0;
        self.left = 0;
        self.right = 0;
        self.same = 0;
        self.value = T::default();
    }
    pub fn value(&self) -> &T {
        &self.value
    }
}

pub enum Removed<T> {
    Last(T),
    Remain,
    None,
}

pub struct Avltriee<T> {
    root: ManuallyDrop<Box<u32>>,
    node_list: ManuallyDrop<Box<AvltrieeNode<T>>>,
}
impl<T: Clone + Default> Avltriee<T> {
    pub fn new(root: *mut u32, node_list: *mut AvltrieeNode<T>) -> Avltriee<T> {
        Avltriee {
            root: ManuallyDrop::new(unsafe { Box::from_raw(root) }),
            node_list: ManuallyDrop::new(unsafe { Box::from_raw(node_list) }),
        }
    }
    pub unsafe fn update(&mut self, row: u32, new_data: T)
    where
        T: Ord,
    {
        if let Some(n) = self.node(row) {
            if n.value().cmp(&new_data) != Ordering::Equal {
                //データが変更なしの場合は何もしない
                self.remove(row); //変更の場合、一旦消してから登録しなおす
                self.update_with_search(row, new_data);
            }
        } else {
            self.update_with_search(row, new_data);
        }
    }
    unsafe fn update_with_search(&mut self, row: u32, data: T)
    where
        T: Ord,
    {
        let (ord, found_row) = self.search(&data);
        if ord == Ordering::Equal && found_row != 0 {
            self.update_same(found_row, row);
        } else {
            self.update_node(found_row, row, data, ord);
            if **self.root == 0 {
                **self.root = row;
            }
        }
    }

    pub unsafe fn update_node(&mut self, origin: u32, target_row: u32, data: T, ord: Ordering) {
        *self.offset_mut(target_row) = AvltrieeNode::new(target_row, origin, data.clone());
        if origin > 0 {
            let p = self.offset_mut(origin);
            //親ノードのL/R更新。比較結果が小さい場合は左、大きい場合は右
            if ord == Ordering::Less {
                p.left = target_row;
            } else {
                p.right = target_row;
            }
            self.balance(origin);
        }
    }

    unsafe fn same_last(&self, row: u32) -> u32 {
        let mut r = row;
        let mut same = self.offset(r);
        while same.same != 0 {
            r = same.same;
            same = self.offset(r);
        }
        r
    }
    pub unsafe fn update_same(&mut self, vertex_row: u32, new_row: u32) {
        let mut vertex = self.offset_mut(vertex_row);
        let mut new_vertex = self.offset_mut(new_row);
        *new_vertex = vertex.clone();
        if new_vertex.parent == 0 {
            **self.root = new_row;
        } else {
            let mut parent = self.offset_mut(new_vertex.parent);
            if parent.left == vertex_row {
                parent.left = new_row;
            } else {
                parent.right = new_row;
            }
        }
        vertex.parent = new_row;
        new_vertex.same = vertex_row;
        if new_vertex.left != 0 {
            self.offset_mut(new_vertex.left).parent = new_row;
        }
        if new_vertex.right != 0 {
            self.offset_mut(new_vertex.right).parent = new_row;
        }
        vertex.left = 0;
        vertex.right = 0;
    }

    pub fn iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, iter::Order::Asc)
    }
    pub fn desc_iter(&self) -> AvltrieeIter<T> {
        AvltrieeIter::new(&self, iter::Order::Desc)
    }
    pub fn iter_by_value_from(&self, min_value: &T) -> AvltrieeIter<T>
    where
        T: Ord,
    {
        let (_, row) = self.search(min_value);
        AvltrieeIter::begin_at(&self, row, iter::Order::Asc)
    }
    pub fn iter_by_value_to<'a>(&'a self, max_value: &'a T) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value_max(&self, max_value)
    }
    pub fn iter_by_value_from_to<'a>(
        &'a self,
        min_value: &'a T,
        max_value: &'a T,
    ) -> AvltrieeRangeIter<T>
    where
        T: Ord,
    {
        AvltrieeRangeIter::new_with_value(&self, min_value, max_value)
    }
    pub fn iter_by_row_from_to(&self, begin: u32, end: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, begin, end)
    }
    pub fn iter_by_row_from(&self, begin: u32) -> AvltrieeIter<T> {
        AvltrieeIter::begin_at(&self, begin, iter::Order::Asc)
    }
    pub fn iter_by_row_to(&self, end: u32) -> AvltrieeRangeIter<T> {
        AvltrieeRangeIter::new(&self, unsafe { self.min(**self.root) }, end)
    }
    pub unsafe fn node<'a>(&self, row: u32) -> Option<&'a AvltrieeNode<T>> {
        let node = self.offset(row);
        if node.height > 0 {
            Some(node)
        } else {
            None
        }
    }
    pub unsafe fn value<'a>(&self, row: u32) -> Option<&'a T> {
        if let Some(v) = self.node(row) {
            Some(&v.value())
        } else {
            None
        }
    }
    pub fn root(&self) -> u32 {
        **self.root
    }
    pub fn init_node(&mut self, data: T, root: u32) {
        unsafe {
            *self.offset_mut(0) = AvltrieeNode::new(0, 0, T::default()); //0ノード
            *self.offset_mut(root) = AvltrieeNode::new(1, 0, data.clone()); //初回追加分
        }
        **self.root = root;
    }

    fn node_list_mut(&mut self) -> *mut AvltrieeNode<T> {
        &mut **self.node_list
    }
    pub(crate) unsafe fn offset<'a>(&self, offset: u32) -> &'a AvltrieeNode<T> {
        &*(&**self.node_list as *const AvltrieeNode<T>).offset(offset as isize)
    }
    pub(crate) unsafe fn offset_mut<'a>(&mut self, offset: u32) -> &'a mut AvltrieeNode<T> {
        &mut *self.node_list_mut().offset(offset as isize)
    }

    fn join_intermediate(parent: &mut AvltrieeNode<T>, remove_target_row: u32, child_row: u32) {
        if parent.right == remove_target_row {
            parent.right = child_row;
        } else if parent.left == remove_target_row {
            parent.left = child_row;
        } else {
            panic!("crash and burn");
        }
    }
    unsafe fn remove_intermediate(&mut self, remove_target: &mut AvltrieeNode<T>) -> (u32, u32) {
        let left_max_row = self.max(remove_target.left);
        let mut left_max = self.offset_mut(left_max_row);
        let left_max_parent_row = left_max.parent;
        let mut left_max_parent = self.offset_mut(left_max_parent_row);

        if remove_target.left != left_max_row {
            //左最大値の親が削除対象の場合はこの処理は不要
            left_max_parent.right = left_max.left;
            if left_max_parent.right != 0 {
                self.offset_mut(left_max_parent.right).parent = left_max_parent_row;
            }
            left_max.left = remove_target.left;
            if left_max.left != 0 {
                self.offset_mut(left_max.left).parent = left_max_row;
            }
        }

        left_max.parent = remove_target.parent;
        left_max.right = remove_target.right;

        let mut right = self.offset_mut(remove_target.right);
        right.parent = left_max_row;

        (left_max_row, left_max_parent_row)
    }
    pub unsafe fn remove(&mut self, target_row: u32) -> Removed<T> {
        let mut ret = Removed::Remain;
        let remove_target = self.offset_mut(target_row);
        if remove_target.height > 0 {
            if remove_target.parent == 0 {
                //rootを削除する場合
                if remove_target.same != 0 {
                    //同じ値のものが存在する場合、それをrootに昇格
                    let same_row = remove_target.same;
                    let same = self.offset_mut(same_row);
                    same.parent = 0;
                    same.left = remove_target.left;
                    same.right = remove_target.right;
                    same.height = remove_target.height;
                    if same.left != 0 {
                        self.offset_mut(same.left).parent = same_row;
                    }
                    if same.right != 0 {
                        self.offset_mut(same.right).parent = same_row;
                    }
                    **self.root = same_row;
                } else {
                    ret = Removed::Last(remove_target.value().clone());
                    if remove_target.left == 0 && remove_target.right == 0 {
                        //唯一のデータが消失する
                        **self.root = 0;
                    } else if remove_target.left == 0 {
                        //左が空いている。右ノードをrootに
                        **self.root = remove_target.right;
                        self.offset_mut(remove_target.right).parent = 0;
                        self.balance(remove_target.right);
                    } else if remove_target.right == 0 {
                        //右が空いている。左ノードをrootに
                        **self.root = remove_target.left;
                        self.offset_mut(remove_target.left).parent = 0;
                        self.balance(remove_target.left);
                    } else {
                        let (left_max_row, left_max_parent_row) =
                            self.remove_intermediate(remove_target);
                        **self.root = left_max_row;
                        if left_max_parent_row == target_row {
                            self.offset_mut(left_max_parent_row).parent = left_max_row;
                            self.balance(left_max_row);
                        } else {
                            self.balance(left_max_parent_row);
                        }
                    }
                }
            } else {
                let mut parent = self.offset_mut(remove_target.parent);
                if parent.same == target_row {
                    //同じ値がある。前後をつなげる
                    parent.same = remove_target.same;
                } else {
                    ret = Removed::Last(remove_target.value().clone());
                    if remove_target.left == 0 && remove_target.right == 0 {
                        //削除対象が末端の場合
                        if parent.right == target_row {
                            parent.right = 0;
                        } else if parent.left == target_row {
                            parent.left = 0;
                        }
                        self.balance(remove_target.parent);
                    } else if remove_target.left == 0 {
                        //左が空いている。右ノードを親に接ぐ
                        Self::join_intermediate(parent, target_row, remove_target.right);
                        if remove_target.right != 0 {
                            self.offset_mut(remove_target.right).parent = remove_target.parent;
                        }
                        self.balance(remove_target.parent);
                    } else if remove_target.right == 0 {
                        //右が空いている。左ノードを親に接ぐ
                        Self::join_intermediate(parent, target_row, remove_target.left);
                        if remove_target.left != 0 {
                            self.offset_mut(remove_target.left).parent = remove_target.parent;
                        }
                        self.balance(remove_target.parent);
                    } else {
                        //削除対象は中間ノード
                        let (left_max_row, left_max_parent_row) =
                            self.remove_intermediate(remove_target);
                        if parent.right == target_row {
                            parent.right = left_max_row;
                        } else {
                            parent.left = left_max_row;
                        }
                        if left_max_parent_row == target_row {
                            self.balance(left_max_row);
                        } else {
                            self.balance(left_max_parent_row);
                        }
                    }
                }
            }
            remove_target.reset();
        }
        ret
    }

    unsafe fn calc_height(&mut self, vertex_row: u32) {
        let mut vertex = self.offset_mut(vertex_row);

        let left = self.offset(vertex.left);
        let right = self.offset(vertex.right);

        vertex.height = std::cmp::max(left.height, right.height) + 1;
    }
    unsafe fn balance(&mut self, vertex_row: u32) {
        let mut vertex_row = vertex_row;
        loop {
            let mut vertex = self.offset_mut(vertex_row);

            let mut parent_row = vertex.parent;

            let left_row = vertex.left;
            let right_row = vertex.right;

            let mut left = self.offset_mut(left_row);
            let mut right = self.offset_mut(right_row);

            let diff = left.height as isize - right.height as isize;
            if diff.abs() >= 2 {
                let high_is_left = diff > 0;

                let new_vertex_row = if high_is_left {
                    self.max(left_row)
                } else {
                    self.min(right_row)
                };
                let new_vertex = self.offset_mut(new_vertex_row);
                let new_vertex_old_parent = new_vertex.parent;
                vertex.parent = new_vertex_row;
                new_vertex.parent = parent_row;
                if parent_row == 0 {
                    **self.root = new_vertex_row;
                } else {
                    let parent = self.offset_mut(parent_row);
                    if parent.left == vertex_row {
                        parent.left = new_vertex_row;
                    } else {
                        parent.right = new_vertex_row;
                    }
                }
                if high_is_left {
                    new_vertex.right = vertex_row;
                    vertex.left = 0;
                    if new_vertex_row == left_row {
                        vertex = self.offset_mut(left_row);
                        left = self.offset_mut(vertex.left);
                        right = self.offset_mut(vertex_row);

                        self.calc_height(vertex.left);
                    } else {
                        let new_left_row = self.min(new_vertex_row);
                        let new_left = self.offset_mut(new_left_row);
                        new_left.left = left_row;

                        left.parent = new_left_row;
                        self.offset_mut(new_vertex_old_parent).right = 0;

                        self.calc_height(left_row);

                        left = self.offset_mut(vertex.left);

                        parent_row = new_vertex_old_parent;
                    }
                    self.calc_height(vertex_row);
                } else {
                    new_vertex.left = vertex_row;
                    vertex.right = 0;
                    if new_vertex_row == right_row {
                        vertex = self.offset_mut(right_row);
                        left = self.offset_mut(vertex_row);
                        right = self.offset_mut(vertex.right);

                        self.calc_height(vertex.right);
                    } else {
                        let new_right_row = self.max(new_vertex_row);
                        let new_right = self.offset_mut(new_right_row);
                        new_right.right = right_row;

                        right.parent = new_right_row;
                        self.offset_mut(new_vertex_old_parent).left = 0;

                        self.calc_height(right_row);

                        right = self.offset_mut(vertex.right);

                        parent_row = new_vertex_old_parent;
                    }
                    self.calc_height(vertex_row);
                }
            }

            vertex.height = std::cmp::max(left.height, right.height) + 1;
            vertex_row = parent_row;
            if vertex_row == 0 {
                //頂点まで遡及完了した場合は抜ける
                break;
            }
        }
    }
    /*
    与えられた値を検索する。
    最終的には左右どちらかが空いているノードが返される事になる
     */
    pub fn search(&self, value: &T) -> (Ordering, u32)
    where
        T: Ord,
    {
        let mut origin = **self.root;
        let mut ord = Ordering::Equal;

        while origin != 0 {
            let p = unsafe { self.offset(origin) };
            ord = value.cmp(&p.value());
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    origin = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    origin = p.right;
                }
            }
        }
        (ord, origin)
    }
    pub fn search_cb<F>(&self, ord_cb: F) -> (Ordering, u32)
    where
        F: Fn(&T) -> Ordering,
    {
        let mut origin = **self.root;
        let mut ord = Ordering::Equal;
        while origin != 0 {
            let p = unsafe { self.offset(origin) };
            ord = ord_cb(&p.value());
            match ord {
                Ordering::Less => {
                    if p.left == 0 {
                        break;
                    }
                    origin = p.left;
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if p.right == 0 {
                        break;
                    }
                    origin = p.right;
                }
            }
        }
        (ord, origin)
    }
    pub unsafe fn sames(&self, same_root: u32) -> Vec<u32> {
        let mut r = Vec::new();
        let mut t = same_root;
        loop {
            let node = self.offset(t);
            if node.same != 0 {
                r.push(node.same.into());
                t = node.same;
            } else {
                break;
            }
        }
        r
    }
    unsafe fn max(&self, t: u32) -> u32 {
        let node = self.offset(t);
        let r = node.right;
        if r == 0 {
            t
        } else {
            self.max(r)
        }
    }
    unsafe fn min(&self, t: u32) -> u32 {
        let node = self.offset(t);
        let l = node.left;
        if l == 0 {
            t
        } else {
            self.min(l)
        }
    }
    unsafe fn retroactive(&self, c: u32) -> Option<u32> {
        let t = self.offset(c);
        let parent = t.parent;
        if parent == 0 {
            if t.right == 0 || t.same == 0 {
                None
            } else {
                Some(t.right)
            }
        } else {
            let parent_node = self.offset(parent);
            if parent_node.right == c {
                if let Some(p) = self.retroactive(parent) {
                    if p != c {
                        Some(p)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                Some(parent)
            }
        }
    }
    unsafe fn same_root(&self, row: u32) -> u32 {
        let mut r = row;
        loop {
            let same = self.offset(r);
            let parent_node = self.offset(same.parent);
            if parent_node.right == r {
                break;
            }
            r = same.parent;
            if parent_node.parent == 0 {
                break;
            }
        }
        r
    }
    pub(crate) unsafe fn next(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let node = self.offset(c);
        let parent_node = self.offset(node.parent);
        if node.same != 0 {
            if parent_node.left == c || parent_node.right == c {
                Some((node.same, c))
            } else {
                Some((node.same, same_branch))
            }
        } else {
            if parent_node.same == c {
                let sr = if same_branch != 0 {
                    same_branch
                } else {
                    self.same_root(node.parent)
                };
                if sr != 0 {
                    if let Some(i) = self.retroactive(sr) {
                        Some((i, 0))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else if parent_node.left == c {
                //対象ノードが親の左の場合
                if node.right != 0 {
                    //自身の右にノードがある場合は右ノードのminを返す
                    Some((self.min(node.right), same_branch))
                } else {
                    //自身の右ノードが無い場合、親と同じ値の最後のデータを返す
                    if parent_node.same == 0 {
                        Some((node.parent, same_branch))
                    } else {
                        Some((self.same_last(node.parent), same_branch))
                    }
                }
            } else if parent_node.right == c {
                //自身が右の場合
                if node.right != 0 {
                    //右ノードがあれば右の最小を返す
                    Some((self.min(node.right), same_branch))
                } else {
                    //右ノードが無い場合、はノードの終端。
                    if parent_node.parent == 0 {
                        None
                    } else {
                        if let Some(i) = self.retroactive(node.parent) {
                            Some((i, same_branch))
                        } else {
                            None
                        }
                    }
                }
            } else {
                //自身がrootの場合、ここに来る場合がある
                if node.right != 0 {
                    //右ノードの最小値を返す
                    Some((self.min(node.right), same_branch))
                } else {
                    None //右も左も親も無い場合は自身が唯一のデータなので次は無い
                }
            }
        }
    }

    unsafe fn retroactive_desc(&self, c: u32) -> Option<u32> {
        let t = self.offset(c);
        let parent = t.parent;
        if parent == 0 {
            if t.left == 0 || t.same == 0 {
                None
            } else {
                Some(t.left)
            }
        } else {
            let parent_node = self.offset(parent);
            if parent_node.left == c {
                if let Some(p) = self.retroactive_desc(parent) {
                    if p != c {
                        Some(p)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                Some(parent)
            }
        }
    }
    pub(crate) unsafe fn next_desc(&self, c: u32, same_branch: u32) -> Option<(u32, u32)> {
        let node = self.offset(c);
        let parent_node = self.offset(node.parent);
        if node.same != 0 {
            if parent_node.left == c || parent_node.right == c {
                Some((node.same, c))
            } else {
                Some((node.same, same_branch))
            }
        } else {
            if parent_node.same == c {
                let sr = if same_branch != 0 {
                    same_branch
                } else {
                    self.same_root(node.parent)
                };
                if sr != 0 {
                    if let Some(i) = self.retroactive_desc(sr) {
                        Some((i, 0))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else if parent_node.right == c {
                if node.left != 0 {
                    Some((self.max(node.left), same_branch))
                } else {
                    if parent_node.same == 0 {
                        Some((node.parent, same_branch))
                    } else {
                        Some((self.same_last(node.parent), same_branch))
                    }
                }
            } else if parent_node.left == c {
                if node.left != 0 {
                    Some((self.max(node.left), same_branch))
                } else {
                    if parent_node.parent == 0 {
                        None
                    } else {
                        if let Some(i) = self.retroactive_desc(node.parent) {
                            Some((i, same_branch))
                        } else {
                            None
                        }
                    }
                }
            } else {
                if node.left != 0 {
                    Some((self.max(node.left), same_branch))
                } else {
                    None
                }
            }
        }
    }
}
