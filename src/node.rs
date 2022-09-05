#[repr(C)]
pub struct TriAVLTreeNode<T>{    //T:実データ型
    pub height: u8
    ,pub parent: i64
    ,pub left: i64
    ,pub right: i64
    ,pub same: i64
    ,pub last: i64    //同一データの場合の終端id(ルートにのみ設定される。ユニークな場合自身のidが入る)
    ,value: T
}   //アドレスは64bitCPUの場合48bitとかになるらしいのでi64にしておく

impl<T: std::fmt::Debug> std::fmt::Debug for TriAVLTreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f
            ,"{{ height:{} , parent:{} , left:{} , right:{} , same:{} , last:{} , value:{:?} }}"
            ,self.height
            ,self.parent
            ,self.left
            ,self.right
            ,self.same
            ,self.last
            ,self.value
        )
    }
}
impl<T> TriAVLTreeNode<T>{
    pub fn new(id:i64,parent:i64,value:T)->TriAVLTreeNode<T>{
        TriAVLTreeNode{
            height:if id==0{0}else{1}
            ,parent
            ,left:0
            ,right:0
            ,same:0
            ,last:id
            ,value
        }
    }
    pub fn new_same(parent:i64,value:T)->TriAVLTreeNode<T> where T:Clone{
        TriAVLTreeNode{
            height:1
            ,parent
            ,left:0
            ,right:0
            ,same:0
            ,last:0
            ,value
        }
    }
    pub fn reset(&mut self) where T : std::default::Default{
        self.height=0;
        self.parent=0;
        self.left=0;
        self.right=0;
        self.same=0;
        self.last=0;
        self.value=T::default();
    }
    pub fn value(&self)->&T{
        &self.value
    }
}