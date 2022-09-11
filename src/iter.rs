use super::AVLTriee;
use super::AVLTrieeNode;

pub struct AVLTrieeIter<'a,T>{
    now:u32
    ,same_branch:u32
    ,local_index:isize
    ,tree:&'a AVLTriee<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTrieeIter<'a,T> {
    type Item = (isize,u32,&'a AVLTrieeNode<T>);
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
impl<'a,T:Clone+Copy+Default> AVLTrieeIter<'a,T>{
    pub fn new(tree:&'a AVLTriee<T>)->AVLTrieeIter<'a,T>{
        AVLTrieeIter{
            now:tree.min(tree.root() as u32)
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
    pub fn begin_at(tree:&'a AVLTriee<T>,begin:u32)->AVLTrieeIter<'a,T>{
        AVLTrieeIter{
            now:begin
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
}

pub struct AVLTrieeRangeIter<'a,T>{
    now:u32
    ,end:u32
    ,same_branch:u32
    ,local_index:isize
    ,tree:&'a AVLTriee<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTrieeRangeIter<'a,T> {
    type Item = (isize,u32,&'a AVLTrieeNode<T>);
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
impl<'a,T:Clone+Copy+Default> AVLTrieeRangeIter<'a,T>{
    pub fn new(tree:&'a AVLTriee<T>,begin:u32,end:u32)->AVLTrieeRangeIter<'a,T>{
        AVLTrieeRangeIter{
            now:begin
            ,end
            ,same_branch:0
            ,local_index:0
            ,tree
        }
    }
}

pub struct AVLTrieeIterSeq<'a,T>{
    now:u32
    ,tree:&'a AVLTriee<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTrieeIterSeq<'a,T> {
    type Item = (u32,&'a AVLTrieeNode<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.now += 1;
        if self.now<=self.tree.record_count(){
            Some((self.now,&self.tree.offset(self.now)))
        }else{
            None
        }
    }
}
impl<'a,T:Clone+Copy+Default> AVLTrieeIterSeq<'a,T>{
    pub fn new(tree:&'a AVLTriee<T>)->AVLTrieeIterSeq<'a,T>{
        AVLTrieeIterSeq{
            now:0
            ,tree
        }
    }
}