use super::TriAVLTree;
use super::TriAVLTreeNode;

pub struct TriAVLTreeIter<'a,T>{
    now:i64
    ,same_branch:i64
    ,local_index:isize
    ,tree:&'a TriAVLTree<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for TriAVLTreeIter<'a,T> {
    type Item = (isize,i64,&'a TriAVLTreeNode<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            match self.tree.next(self.now,self.same_branch){
                Some((i,b))=>{
                    self.now=i;
                    self.same_branch=b;
                }
                ,_=>{
                    self.now=0;
                }
            }
            Some((self.local_index,c,&self.tree.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default> TriAVLTreeIter<'a,T>{
    pub fn new(tree:&'a TriAVLTree<T>)->TriAVLTreeIter<'a,T>{
        TriAVLTreeIter{
            now:tree.min(tree.root())
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
    pub fn begin_at(tree:&'a TriAVLTree<T>,begin:i64)->TriAVLTreeIter<'a,T>{
        TriAVLTreeIter{
            now:begin
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
}

pub struct AVLTreeRangeIter<'a,T>{
    now:i64
    ,end:i64
    ,same_branch:i64
    ,local_index:isize
    ,tree:&'a TriAVLTree<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTreeRangeIter<'a,T> {
    type Item = (isize,i64,&'a TriAVLTreeNode<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            if self.now==self.end{
                self.now=0;
            }else{
                match self.tree.next(self.now,self.same_branch){
                    Some((i,b))=>{
                        self.now=i;
                        self.same_branch=b;
                    }
                    ,_=>{
                        self.now=0;
                    }
                }
            }
            Some((self.local_index,c,&self.tree.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default> AVLTreeRangeIter<'a,T>{
    pub fn new(tree:&'a TriAVLTree<T>,begin:i64,end:i64)->AVLTreeRangeIter<'a,T>{
        AVLTreeRangeIter{
            now:begin
            ,end
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
}

pub struct AVLTreeIterSeq<'a,T>{
    now:i64
    ,tree:&'a TriAVLTree<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTreeIterSeq<'a,T> {
    type Item = (i64,&'a TriAVLTreeNode<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.now += 1;
        if self.now<=self.tree.record_count() as i64{
            Some((self.now,&self.tree.offset(self.now)))
        }else{
            None
        }
    }
}
impl<'a,T:Clone+Copy+Default> AVLTreeIterSeq<'a,T>{
    pub fn new(tree:&'a TriAVLTree<T>)->AVLTreeIterSeq<'a,T>{
        AVLTreeIterSeq{
            now:0
            ,tree
        }
    }
}