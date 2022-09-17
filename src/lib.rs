use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::collections::HashSet;
use std::cmp::Ordering;

mod iter;
use iter::AVLTrieeIter;
use iter::AVLTrieeRangeIter;
use iter::AVLTrieeIterSeq;

#[derive(Clone)]
pub struct AVLTrieeNode<T>{
    parent: u32
    ,left: u32
    ,right: u32
    ,same: u32
    ,height: u8
    ,value: T    //T:実データ型
}   //最大行数はu32の最大値となる。64bitCPUが扱えるアドレス的には不足だけど行当たり8バイトを超える時点で32bit行以内にアドレッシング出来くなくなりそうなのでヨシ
impl<T: std::fmt::Debug> std::fmt::Debug for AVLTrieeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f
            ,"{{ height:{} , parent:{} , left:{} , right:{} , same:{} , value:{:?} }}"
            ,self.height
            ,self.parent
            ,self.left
            ,self.right
            ,self.same
            ,self.value
        )
    }
}
impl<T> AVLTrieeNode<T>{
    pub fn new(id:u32,parent:u32,value:T)->AVLTrieeNode<T>{
        AVLTrieeNode{
            height:if id==0{0}else{1}
            ,parent
            ,left:0
            ,right:0
            ,same:0
            ,value
        }
    }
    pub fn reset(&mut self) where T : std::default::Default{
        self.height=0;
        self.parent=0;
        self.left=0;
        self.right=0;
        self.same=0;
        self.value=T::default();
    }
    pub fn value(&self)->&T{
        &self.value
    }
    pub fn parent(&self)->u32{
        self.parent
    }
    pub fn left(&self)->u32{
        self.left
    }
    pub fn right(&self)->u32{
        self.right
    }
    pub fn same(&self)->u32{
        self.same
    }
}

pub enum RemoveResult<T>{
    Unique(T)
    ,NotUnique
}

pub type IdSet = HashSet<i64,BuildHasherDefault<FxHasher>>;

pub struct AVLTriee<T>{
    root: *mut u32
    ,node_list: *mut AVLTrieeNode<T>
    ,record_count:u32
}
impl<T: std::marker::Copy +  std::clone::Clone + std::default::Default> AVLTriee<T>{
    pub fn new(
        root: *mut u32
        ,node_list: *mut AVLTrieeNode<T>
        ,record_count:u32
    )->AVLTriee<T>{
        AVLTriee{
            root
            ,node_list
            ,record_count
        }
    }
    
    pub fn insert(&mut self,data:T) where T:std::cmp::Ord{
        self.record_count+=1;
        self.insert_new(self.record_count,data);
    }
    pub fn update(&mut self,id:u32,new_data:T) where T:std::cmp::Ord{
        if let Some(n)=self.node(id){
            if n.height==0{ //新規登録
                if self.record_count==id{
                    self.insert_new(self.record_count,new_data);
                }else{  //id使いまわし
                    self.update_with_search(id,new_data);
                }
            }else{
                if n.value().cmp(&new_data)!=Ordering::Equal{  //データが変更なしの場合は何もしない
                    self.remove(id);   //変更の場合、一旦消してから登録しなおす
                    self.update_with_search(id,new_data);
                }
            }
        }
    }

    fn insert_new(&mut self,id:u32,data:T) where T:std::cmp::Ord{
        if self.root()==0{  //初回登録
            self.init_node(data);
        }else{
            self.update_with_search(id,data);
        }
    }
    fn update_with_search(&mut self,id:u32,data:T) where T:std::cmp::Ord{
        let (ord,found_id)=self.search(&data);
        if ord==Ordering::Equal && found_id!=0{
            self.update_same(found_id,id);
        }else{
            self.update_node(found_id,id,data,ord);
        }
    }

    pub fn update_node(&mut self,origin:u32,new_id:u32,data:T,ord:Ordering) where T:Copy{
        unsafe{
            *self.node_list.offset(new_id as isize)=AVLTrieeNode::new(new_id,origin,data);    //とりあえず終端の子として作る(起点ノード)
        }
        let p=self.offset_mut(origin);
        //親ノードのL/R更新。比較結果が小さい場合は左、大きい場合は右
        if ord==Ordering::Less{
            p.left=new_id;
        }else{
            p.right=new_id;
        }
        self.balance(origin);
    }

    pub fn same_last(&self,node_id:u32)->u32{
        let mut r=node_id;
        let mut same=self.offset(r);
        while same.same!=0{
            r=same.same;
            same=self.offset(r);
        }
        r
    }
    pub fn update_same(&mut self,vertex_id:u32,new_id:u32){
        let mut vertex=self.offset_mut(vertex_id);
        let mut new_vertex=self.offset_mut(new_id);
        *new_vertex=vertex.clone();
        if new_vertex.parent==0{
            unsafe{*self.root=new_id;}
        }else{
            let mut parent=self.offset_mut(new_vertex.parent);
            if parent.left==vertex_id{
                parent.left=new_id;
            }else{
                parent.right=new_id;
            }
        }
        vertex.parent=new_id;
        new_vertex.same=vertex_id;

        vertex.left=0;
        vertex.right=0;
    }

    pub fn iter(&self)->AVLTrieeIter<T>{
        AVLTrieeIter::new(&self)
    }
    pub fn iter_value_from(&self,min_value:&T)->AVLTrieeIter<T> where T:std::cmp::Ord{
        let (_,id)=self.search(min_value);
        AVLTrieeIter::begin_at(&self,id)
    }
    pub fn iter_begin_at(&self,begin:u32)->AVLTrieeIter<T>{
        AVLTrieeIter::begin_at(&self,begin)
    }
    pub fn iter_range<'a>(&'a self,min_value:&'a T,max_value:&'a T)->AVLTrieeRangeIter<T> where T:std::cmp::Ord{
        AVLTrieeRangeIter::new(&self,min_value,max_value)
    }
    pub fn iter_seq(&self)->AVLTrieeIterSeq<T>{
        AVLTrieeIterSeq::new(&self)
    }
    pub fn node<'a>(&self,id:u32) ->Option<&'a AVLTrieeNode<T>>{
        if (self.record_count())<id{
            None    //存在しないidが指定されている場合はNoneを返す
        }else{
            Some(&self.offset(id))
        }
    }
    pub fn entity_value<'a>(&self,id:u32)->Option<&'a T>{
        if let Some(v)=self.node(id){
            Some(&v.value())
        }else{
            None
        }
    }
    pub fn record_count(&self)->u32{
        self.record_count
    }
    pub fn set_record_count(&mut self,c:u32){
        self.record_count=c;
    }
    pub fn add_record_count(&mut self,c:u32){
        self.record_count+=c;
    }
    pub fn root(&self)->u32{
        unsafe{*self.root}
    }
    
    pub fn init_node(&mut self,data:T) where T:Default+Copy{
        unsafe{
            *self.node_list=AVLTrieeNode::new(0,0,T::default()); //0ノード
            (*self.node_list.offset(1))=AVLTrieeNode::new(1,0,data); //初回追加分
            *self.root=1;
        }
        self.record_count=1;
    }
    
    pub fn offset<'a>(&self,offset:u32)->&'a AVLTrieeNode<T>{
        unsafe{&*self.node_list.wrapping_offset(offset as isize)}
    }
    pub fn offset_mut<'a>(&mut self,offset:u32)->&'a mut AVLTrieeNode<T>{
        unsafe{&mut *self.node_list.wrapping_offset(offset as isize)}
    }

    fn join_intermediate(parent:&mut AVLTrieeNode<T>,remove_target_id:u32,child_id:u32){
        if parent.right==remove_target_id{
            parent.right=child_id;
        }else if parent.left==remove_target_id{
            parent.left=child_id;
        }else{
            panic!("crash and burn"); 
        }
    }
    fn remove_intermediate(&mut self,remove_target:&mut AVLTrieeNode<T>)->(u32,u32){
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
    pub fn remove(&mut self,target_id:u32)->RemoveResult<T> where T:Default+Clone{
        let mut ret=RemoveResult::NotUnique;
        let remove_target=self.offset_mut(target_id);
        if remove_target.height>0{
            if remove_target.parent==0{ //rootを削除する場合
                if remove_target.same!=0{
                    //同じ値のものが存在する場合、それをrootに昇格
                    let same_id=remove_target.same;
                    let same=self.offset_mut(same_id);
                    same.left=remove_target.left;
                    same.right=remove_target.right;
                    unsafe{*self.root=same_id}
                }else{
                    ret=RemoveResult::Unique(remove_target.value().clone());
                    if remove_target.left==0 && remove_target.right==0{
                        //唯一のデータが消失する
                        unsafe{*self.root=0}
                    }else if remove_target.left==0{
                        //左が空いている。右ノードをrootに
                        unsafe{*self.root=remove_target.right}
                        self.offset_mut(remove_target.right).parent=0;
                        self.balance(remove_target.right);
                    }else if remove_target.right==0{
                        //右が空いている。左ノードをrootに
                        unsafe{*self.root=remove_target.left}
                        self.offset_mut(remove_target.left).parent=0;
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
                    parent.same=remove_target.same;
                }else{
                    ret=RemoveResult::Unique(remove_target.value().clone());
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
        }
        ret
    }

    fn calc_height(&mut self,vertex_id:u32){
        let mut vertex=self.offset_mut(vertex_id);

        let left=self.offset(vertex.left);
        let right=self.offset(vertex.right);

        vertex.height=std::cmp::max(
            left.height
            ,right.height
        )+1;
    }
    fn balance(&mut self,vertex_id:u32){
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

                        left=self.offset_mut(vertex.left);

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

                        right=self.offset_mut(vertex.right);

                        parent_id=new_vertex_old_parent;
                    }
                    self.calc_height(vertex_id);
                }
            }

            vertex.height=std::cmp::max(
                left.height
                ,right.height
            )+1;    //左右のノードの高い方の高さ+1
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
    pub fn search(&self,target:&T)->(Ordering,u32) where T:Ord{
        let mut origin=self.root() as u32;
        let mut ord=Ordering::Equal;

        while origin!=0{
            let p=self.offset(origin);
            ord=target.cmp(&p.value());
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
    pub fn search_cb<F>(&self,ord_cb:F)->(Ordering,u32) where F:Fn(&T)->Ordering{
        let mut origin=self.root() as u32;
        let mut ord=Ordering::Equal;

        while origin!=0{
            let p=self.offset(origin);
            ord=ord_cb(&p.value());
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
    pub fn sames(&self,result:&mut IdSet,t:u32){
        let mut t=t;
        loop{
            let node=self.offset(t);
            if node.same!=0{
                result.insert(node.same.into());
                t=node.same;
            }else{
                break;
            }
        }
    }
    pub fn sames_and(&self,result:&mut IdSet,and:&IdSet,t:u32){
        let mut t=t;
        loop{
            let node=self.offset(t);
            if node.same!=0{
                let same=node.same as i64;
                if and.contains(&same){
                    result.insert(same);
                }
                t=node.same;
            }else{
                break;
            }
        }
    }
    fn max(&self,t:u32)->u32{
        let node=self.offset(t);
        let r=node.right;
        if r==0{
            t
        }else{
            self.max(r)
        }
    }
    fn min(&self,t:u32)->u32{
        let node=self.offset(t);
        let l=node.left;
        if l==0{
            t
        }else{
            self.min(l)
        }
    }
    fn retroactive(&self,c:u32)->Option<u32>{
        let parent=self.offset(c).parent;
        if parent==0{
            None
        }else{
            let parent_node=self.offset(parent);
            if parent_node.right==c{    //自身が右の場合、さらに大きいの値が上にある
                self.retroactive(parent)
            }else{  //自身が左の場合、
                Some(parent)
            }
        }
    }
    fn same_root(&self,node_id:u32)->u32{
        let mut r=node_id;
        loop {
            let same=self.offset(r);
            let parent_node=self.offset(same.parent);
            if parent_node.right==r{
                break;
            }
            r=same.parent;
        }
        r
    }
    pub fn next(&self,c:u32,same_branch:u32)->Option<(u32,u32)>{
        let node=self.offset(c);
        let parent_node=self.offset(node.parent);
        if node.same!=0{
            if parent_node.left==c || parent_node.right==c{
                Some((node.same,c))
            }else{
                Some((node.same,same_branch))
            }
        }else{
            if parent_node.same==c{
                let sr=if same_branch!=0{
                    same_branch
                }else{
                    self.same_root(node.parent)
                };
                if sr!=0{
                    if let Some(i)=self.retroactive(sr){
                        Some((i,0))
                    }else{
                        None
                    }
                }else{
                    None
                }
            }else if parent_node.left==c{ //対象ノードが親の左の場合
                if node.right!=0{
                    //自身の右にノードがある場合は右ノードのminを返す
                    Some((self.min(node.right),same_branch))
                }else{
                    //自身の右ノードが無い場合、親と同じ値の最後のデータを返す
                    if parent_node.same==0{
                        Some((node.parent,same_branch))
                    }else{
                        Some((self.same_last(node.parent),same_branch))
                    }
                }
            }else if parent_node.right==c{    //自身が右の場合
                if node.right!=0{
                    //右ノードがあれば右の最小を返す
                    Some((self.min(node.right),same_branch))
                }else{  //右ノードが無い場合、はノードの終端。
                    if parent_node.parent==0{
                        //親が無い場合
                        if parent_node.right!=0{
                            Some((self.min(parent_node.right),same_branch))
                        }else{
                            None
                        }
                    }else{
                        if let Some(i)=self.retroactive(node.parent){
                            Some((i,same_branch))
                        }else{
                            None
                        }
                    }
                }
            }else{
                //自身がrootの場合、ここに来る場合がある
                if node.right!=0{   //右ノードの最小値を返す
                    Some((self.min(node.right),same_branch))
                }else{
                    None    //右も左も親も無い場合は自身が唯一のデータなので次は無い
                }
            }
        }
    }
}
