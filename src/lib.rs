use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::collections::HashSet;
use std::cmp::Ordering;

pub mod node;
use node::TriAVLTreeNode;

mod iter;
use iter::TriAVLTreeIter;
use iter::AVLTreeRangeIter;
use iter::AVLTreeIterSeq;

pub enum RemoveResult<T>{
    Unique(T)
    ,NotUnique
}

pub type IdSet = HashSet<i64,BuildHasherDefault<FxHasher>>;

pub struct TriAVLTree<T>{
    root: *mut i64
    ,node_list: *mut TriAVLTreeNode<T>
    ,record_count:usize
}
impl<T: std::marker::Copy +  std::clone::Clone + std::default::Default> TriAVLTree<T>{
    pub fn new(
        root: *mut i64
        ,node_list: *mut TriAVLTreeNode<T>
        ,record_count:usize
    )->TriAVLTree<T>{
        TriAVLTree{
            root
            ,node_list
            ,record_count
        }
    }
    
    pub fn insert(&mut self,data:T) where T:std::cmp::Ord{
        self.record_count+=1;
        self.insert_new(self.record_count as i64,data);
    }
    pub fn update(&mut self,id:i64,newdata:T) where T:std::cmp::Ord{
        if let Some(n)=self.node(id){
            if n.height==0{ //新規登録
                self.insert_new(self.record_count as i64,newdata);
            }else{
                if n.data().cmp(&newdata)!=Ordering::Equal{  //データが変更なしの場合は何もしない
                    self.remove(id);   //変更の場合、一旦消してから登録しなおす
                    self.update_with_search(id,newdata);
                }
            }
        }
    }

    fn insert_new(&mut self,id:i64,data:T) where T:std::cmp::Ord{
        if self.root()==0{  //初回登録
            self.init_node(data);
        }else{
            self.update_with_search(id,data);
        }
    }
    fn update_with_search(&mut self,id:i64,data:T) where T:std::cmp::Ord{
        let (ord,found_id)=self.search(&data);
        if ord==Ordering::Equal && found_id!=0{
            self.update_same(found_id,id,data);
        }else{
            self.update_node(found_id,id,data,ord);
        }
    }

    pub fn update_node(&mut self,origin:i64,newid:i64,data:T,ord:Ordering) where T:Copy{
        unsafe{        
            *self.node_list.offset(newid as isize)=TriAVLTreeNode::new(newid,origin,data);    //とりあえず終端の子として作る(起点ノード)
        }
        let p=self.offset_mut(origin);
        //親ノードのL/R更新。比較結果が小さい場合は左、大きい場合は右
        if ord==Ordering::Less{
            p.left=newid;
        }else{
            p.right=newid;
        }
        self.balance(origin);
    }
    pub fn update_same(&mut self,sames_root:i64,newid:i64,data:T) where T:Clone{
        let mut sames_root=self.offset_mut(sames_root);
        let last=sames_root.last; //同一データのルートのlastに同一データの最終のidが入っているので取っておく
        unsafe{
            *self.node_list.offset(newid as isize)=TriAVLTreeNode::new_same(last,data);
        }
        self.offset_mut(last).same=newid; //最終データのsameを新しく追加されたidに（追加される度に繋がっていく）
        sames_root.last=newid; //ルートのlastを新しく追加されたidに
    }

    pub fn iter(&self)->TriAVLTreeIter<T>{
        TriAVLTreeIter::new(&self)
    }
    pub fn iter_begin_at(&self,begin:i64)->TriAVLTreeIter<T>{
        TriAVLTreeIter::begin_at(&self,begin)
    }
    pub fn iter_range(&self,begin:i64,end:i64)->AVLTreeRangeIter<T>{
        AVLTreeRangeIter::new(&self,begin,end)
    }

    pub fn iter_seq(&self)->AVLTreeIterSeq<T>{
        AVLTreeIterSeq::new(&self)
    }
    pub fn node<'a>(&self,id:i64) ->Option<&'a TriAVLTreeNode<T>>{
        if (self.record_count() as i64)<id{
            None    //存在しないidが指定されている場合はNoneを返す
        }else{
            Some(&self.offset(id))
        }
    }
    pub fn entity_data<'a>(&self,id:i64)->Option<&'a T>{
        if let Some(v)=self.node(id){
            Some(&v.data())
        }else{
            None
        }
    }
    pub fn record_count(&self)->usize{
        self.record_count
    }
    pub fn set_record_count(&mut self,c:usize){
        self.record_count=c;
    }
    pub fn add_record_count(&mut self,c:usize){
        self.record_count+=c;
    }
    pub fn root(&self)->i64{
        unsafe{*self.root}
    }
    
    pub fn init_node(&mut self,data:T) where T:Default+Copy{
        unsafe{
            *self.node_list=TriAVLTreeNode::new(0,0,T::default()); //0ノード
            (*self.node_list.offset(1))=TriAVLTreeNode::new(1,0,data); //初回追加分
            *self.root=1;
        }
        self.record_count=1;
    }
    
    pub fn offset<'a>(&self,offset:i64)->&'a TriAVLTreeNode<T>{
        if offset==0 {
            unsafe{&*self.node_list}
        }else{
            unsafe{&*self.node_list.wrapping_offset(offset as isize)}
        }
    }
    pub fn offset_mut<'a>(&mut self,offset:i64)->&'a mut TriAVLTreeNode<T>{
        unsafe{&mut *self.node_list.wrapping_offset(offset as isize)}
    }

    fn same_root<'a>(&mut self,t:&'a mut TriAVLTreeNode<T>)->&'a mut TriAVLTreeNode<T>{
        let mut root=t;
        while root.last==0{   //rootを探す
            root=self.offset_mut(root.parent);
        }
        root
    }
    fn join_intermediate(parent:&mut TriAVLTreeNode<T>,remove_target_id:i64,child_id:i64){
        if parent.right==remove_target_id{
            parent.right=child_id;
        }else if parent.left==remove_target_id{
            parent.left=child_id;
        }else{
            panic!("crash and burn"); 
        }
    }
    fn remove_intermediate(&mut self,remove_target:&mut TriAVLTreeNode<T>)->(i64,i64){
        let left_max_id=self.max(remove_target.left);
        let mut left_max=self.offset_mut(left_max_id);
        let left_max_parent_id=left_max.parent;
        let mut left_max_parent=self.offset_mut(left_max_parent_id);

        if remove_target.left!=left_max_id{
            //左最大値の親が削除対象の場合はこの処理は不要
            left_max_parent.right=left_max.left;
            left_max.left=remove_target.left;
        }

        left_max.parent=remove_target.parent;
        left_max.right=remove_target.right;

        let mut right=self.offset_mut(remove_target.right);
        right.parent=left_max_id;
        
        (left_max_id,left_max_parent_id)
    }
    pub fn remove(&mut self,target_id:i64)->RemoveResult<T> where T:Default+Clone{
        if self.record_count<(target_id as usize){
            RemoveResult::NotUnique
        }else{
            let mut ret=RemoveResult::NotUnique;
            let remove_target=self.offset_mut(target_id);
            if remove_target.parent==0{ //rootを削除する場合
                if remove_target.same!=0{
                    //同じ値のものが存在する場合、それをrootに昇格
                    let same_id=remove_target.same;
                    let same=self.offset_mut(same_id);
                    same.last=remove_target.last;
                    same.left=remove_target.left;
                    same.right=remove_target.right;
                    unsafe{*self.root=same_id}
                }else{
                    ret=RemoveResult::Unique(remove_target.data().clone());
                    if remove_target.left==0 && remove_target.right==0{
                        //唯一のデータが消失する
                        unsafe{*self.root=0}
                    }else if remove_target.left==0{
                        //左が空いている。右ノードをrootに
                        unsafe{*self.root=remove_target.right}
                        self.balance(remove_target.right);
                    }else if remove_target.right==0{
                        //右が空いている。左ノードをrootに
                        unsafe{*self.root=remove_target.left}
                        self.balance(remove_target.left);
                    }else{
                        let (left_max_id,left_max_parent_id)=self.remove_intermediate(remove_target);
                        unsafe{*self.root=left_max_id}
                        if left_max_parent_id==target_id{
                            self.balance(left_max_id);
                        }else{
                            self.balance(left_max_parent_id);
                        }
                    }
                }
            }else{
                let mut parent=self.offset_mut(remove_target.parent);
                if parent.same==target_id{ //同じ値がある。前後をつなげる
                    if parent.last==target_id{ //削除対象が終端かつ、rootの子が削除対象のみの場合、親は独立データとなる
                        parent.last=remove_target.parent;   //lastは自身を指す
                        parent.same=0;  //同じ値のものは無くなる
                    }else{
                        let mut root=self.same_root(parent);
                        if root.last==target_id{
                            //削除対象は末端データ。
                            root.last=remove_target.parent;
                            parent.same=0;
                        }else{
                            //削除対象は中間データ。親に子データを接ぐ
                            parent.same=remove_target.same;
                            self.offset_mut(remove_target.same).parent=remove_target.parent;
                        }
                    }
                }else{
                    ret=RemoveResult::Unique(remove_target.data().clone());
                    if remove_target.left==0 && remove_target.right==0{
                        //削除対象が末端の場合
                        if parent.right==target_id{
                            parent.right=0;
                        }else if parent.left==target_id{
                            parent.left=0;
                        }
                        self.balance(remove_target.parent);
                    }else if remove_target.left==0{
                        //左が空いている。右ノードを親に接ぐ
                        Self::join_intermediate(parent,target_id,remove_target.right);
                        if remove_target.right!=0{
                            self.offset_mut(remove_target.right).parent=remove_target.parent;
                        }
                        self.balance(remove_target.parent);
                    }else if remove_target.right==0{
                        //右が空いている。左ノードを親に接ぐ
                        Self::join_intermediate(parent,target_id,remove_target.left);
                        if remove_target.left!=0{
                            self.offset_mut(remove_target.left).parent=remove_target.parent;
                        }
                        self.balance(remove_target.parent);
                    }else{
                        //削除対象は中間ノード
                        let (left_max_id,left_max_parent_id)=self.remove_intermediate(remove_target);
                        if parent.right==target_id{
                            parent.right=left_max_id;
                        }else{
                            parent.left=left_max_id;
                        }
                        if left_max_parent_id==target_id{
                            self.balance(left_max_id);
                        }else{
                            self.balance(left_max_parent_id);
                        }
                    }
                }
            }
            remove_target.reset();
            ret
        }
    }
   

    fn calc_height(&mut self,vertex_id:i64){
        let mut vertex=self.offset_mut(vertex_id);

        let left=self.offset(vertex.left);
        let right=self.offset(vertex.right);

        vertex.height=std::cmp::max(
            left.height
            ,right.height
        )+1;
    }
    fn balance(&mut self,vertex_id:i64){
        let mut vertex_id=vertex_id;
        loop {
            let mut vertex=self.offset_mut(vertex_id);

            let mut parent_id=vertex.parent;

            let left_id=vertex.left;
            let right_id=vertex.right;

            let mut left=self.offset_mut(left_id);
            let mut right=self.offset_mut(right_id);

            let diff=left.height as isize  - right.height as isize;
            if diff.abs()>=2{
                let high_is_left=diff>0;

                let new_vertex_id=if high_is_left{
                    self.max(left_id)
                }else{
                    self.min(right_id)
                };
                let new_vertex=self.offset_mut(new_vertex_id);
                let new_vertex_old_parent=new_vertex.parent;
                vertex.parent=new_vertex_id;
                new_vertex.parent=parent_id;
                if parent_id==0{ 
                    unsafe{*self.root=new_vertex_id;}
                }else{
                    let parent=self.offset_mut(parent_id);
                    if parent.left==vertex_id{
                        parent.left=new_vertex_id;
                    }else{
                        parent.right=new_vertex_id;
                    }
                }
                if high_is_left{
                    new_vertex.right=vertex_id;
                    vertex.left=0;
                    if new_vertex_id==left_id{
                        vertex=self.offset_mut(left_id);
                        left=self.offset_mut(vertex.left);
                        right=self.offset_mut(vertex_id);

                        self.calc_height(vertex.left);
                    }else{
                        let new_left=self.offset_mut(self.min(new_vertex_id));
                        new_left.left=left_id;

                        left.parent=new_vertex_id;
                        self.offset_mut(new_vertex_old_parent).right=0;
 
                        self.calc_height(left_id);

                        left=self.offset_mut(new_vertex.left);

                        parent_id=new_vertex_old_parent;
                    }
                    self.calc_height(vertex_id);
                }else{
                    new_vertex.left=vertex_id;
                    vertex.right=0;
                    if new_vertex_id==right_id{
                        vertex=self.offset_mut(right_id);
                        left=self.offset_mut(vertex_id);
                        right=self.offset_mut(vertex.right);

                        self.calc_height(vertex.right);
                    }else{
                        let new_right=self.offset_mut(self.max(new_vertex_id));
                        new_right.right=right_id;

                        right.parent=new_vertex_id;
                        self.offset_mut(new_vertex_old_parent).left=0;

                        self.calc_height(right_id);

                        right=self.offset_mut(new_vertex.right);

                        parent_id=new_vertex_old_parent;
                    }
                    self.calc_height(vertex_id);
                }
            }

            vertex.height=std::cmp::max(
                left.height
                ,right.height
            )+1;    //左右のノードの高い方の高さ+1
            if vertex.parent!=0{
                assert_ne!(vertex.parent,vertex.left);
                assert_ne!(vertex.parent,vertex.right);
            }
            vertex_id=parent_id;
            if vertex_id==0{    //頂点まで遡及完了した場合は抜ける
                break;
            }
        }
    }
    /*
    与えられた値を検索する。
    最終的には左右どちらかが空いているノードが返される事になる
     */
    pub fn search(&self,target:&T)->(Ordering,i64) where T:Ord{
        let mut origin=self.root();
        let mut ord=Ordering::Equal;

        while origin!=0{
            let p=self.offset(origin);
            ord=target.cmp(&p.data());
            match ord{
                Ordering::Less=>{
                    if p.left==0{
                        break;
                    }
                    origin=p.left;
                }
                ,Ordering::Equal=>{
                    break;
                }
                ,Ordering::Greater=>{
                    if p.right==0{
                        break;
                    }
                    origin=p.right;
                }
            }
        }
        (ord,origin)
    }
    pub fn search_cb<F>(&self,ord_cb:F)->(Ordering,i64) where F:Fn(&T)->Ordering{
        let mut origin=self.root();
        let mut ord=Ordering::Equal;

        while origin!=0{
            let p=self.offset(origin);
            ord=ord_cb(&p.data());
            match ord{
                Ordering::Less=>{
                    if p.left==0{
                        break;
                    }
                    origin=p.left;
                }
                ,Ordering::Equal=>{
                    break;
                }
                ,Ordering::Greater=>{
                    if p.right==0{
                        break;
                    }
                    origin=p.right;
                }
            }
        }
        (ord,origin)
    }
    pub fn sames(&self,result:&mut IdSet,t:i64){
        let mut t=t;
        loop{
            let node=self.offset(t);
            if node.same!=0{
                result.insert(node.same);
                t=node.same;
            }else{
                break;
            }
        }
    }
    pub fn sames_and(&self,result:&mut IdSet,and:&IdSet,t:i64){
        let mut t=t;
        loop{
            let node=self.offset(t);
            if node.same!=0{
                if and.contains(&node.same){
                    result.insert(node.same);
                }
                t=node.same;
            }else{
                break;
            }
        }
    }
    fn max(&self,t:i64)->i64{
        let node=self.offset(t);
        let r=node.right;
        if r==0{
            node.last
        }else{
            self.max(r)
        }
    }
    fn min(&self,t:i64)->i64{
        let node=self.offset(t);
        let l=node.left;
        if l==0{
            node.last
        }else{
            self.min(l)
        }
    }
    fn retroactive(&self,c:i64)->Option<i64>{
        let parent=self.offset(c).parent;
        if parent==0{
            None
        }else{
            let parent_node=self.offset(parent);
            if parent_node.right==c{    //自身が右の場合、さらに大きいの値が上にある
                self.retroactive(parent)
            }else{  //自身が左の場合、
                Some(parent_node.last)
            }
        }
    }
    pub fn next(&self,c:i64)->Option<i64>{
        let node=self.offset(c);
        let parent_node=self.offset(node.parent);
        if parent_node.same==c{
            //親と同じ値の場合は親を返す
            Some(node.parent)   
        }else if parent_node.left==c{ //対象ノードが親の左の場合
            if node.right!=0{
                //自身の右にノードがある場合は右ノードのminを返す
                Some(self.min(node.right))
            }else{
                //自身の右ノードが無い場合、親と同じ値の最後のデータを返す
                Some(parent_node.last)
            }
        }else if parent_node.right==c{    //自身が右の場合
            if node.right!=0{
                //右ノードがあれば右の最小を返す
                Some(self.min(node.right))
            }else{  //右ノードが無い場合、はノードの終端。
                if parent_node.parent==0{
                    //親が無い場合
                    if parent_node.right!=0{
                        Some(self.min(parent_node.right))
                    }else{
                        None
                    }
                }else{
                    self.retroactive(node.parent)
                }
            }
        }else{
            //自身がrootの場合、ここに来る場合がある
            if node.right!=0{   //右ノードの最小値を返す
                Some(self.min(node.right))
            }else{
                None    //右も左も親も無い場合は自身が唯一のデータなので次は無い
            }
        }
    }
}