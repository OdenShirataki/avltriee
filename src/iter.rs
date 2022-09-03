use super::TriAVLTree;
use super::node::TriAVLTreeNode;

pub struct TriAVLTreeIter<'a,T>{
    now:i64
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
            self.now=if let Some(i)=self.tree.next(self.now){
                i
            }else{
                0
            };
            Some((self.local_index,c,&self.tree.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default> TriAVLTreeIter<'a,T>{
    pub fn new(tree:&'a TriAVLTree<T>)->TriAVLTreeIter<'a,T>{
        TriAVLTreeIter{
            now:tree.min(tree.root())
            ,local_index:0
            ,tree
        }
    }
    pub fn begin_at(tree:&'a TriAVLTree<T>,begin:i64)->TriAVLTreeIter<'a,T>{
        TriAVLTreeIter{
            now:begin
            ,local_index:0
            ,tree
        }
    }
}

pub struct AVLTreeRangeIter<'a,T>{
    now:i64
    ,end:i64
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
            self.now=if self.now==self.end{  
                0
            }else if let Some(i)=self.tree.next(self.now){
                i
            }else{
                0
            };
            Some((self.local_index,c,&self.tree.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default> AVLTreeRangeIter<'a,T>{
    pub fn new(tree:&'a TriAVLTree<T>,begin:i64,end:i64)->AVLTreeRangeIter<'a,T>{
        AVLTreeRangeIter{
            now:begin
            ,end
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