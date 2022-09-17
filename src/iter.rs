use super::AVLTriee;
use super::AVLTrieeNode;

pub struct AVLTrieeIter<'a,T>{
    now:u32
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a AVLTriee<T>
}
impl<'a,T:Clone+Copy+Default> Iterator for AVLTrieeIter<'a,T> {
    type Item = (isize,u32,&'a AVLTrieeNode<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            match self.triee.next(self.now,self.same_branch){
                Some((i,b))=>{
                    self.now=i;
                    self.same_branch=b;
                }
                ,_=>{
                    self.now=0;
                }
            }
            Some((self.local_index,c,&self.triee.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default> AVLTrieeIter<'a,T>{
    pub fn new(triee:&'a AVLTriee<T>)->AVLTrieeIter<'a,T>{
        AVLTrieeIter{
            now:triee.min(triee.root() as u32)
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn begin_at(triee:&'a AVLTriee<T>,begin:u32)->AVLTrieeIter<'a,T>{
        AVLTrieeIter{
            now:begin
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
}

pub struct AVLTrieeRangeIter<'a,T>{
    now:u32
    ,max_value:&'a T
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a AVLTriee<T>
}
impl<'a,T:Clone+Copy+Default+std::cmp::Ord> Iterator for AVLTrieeRangeIter<'a,T> {
    type Item = (isize,u32,&'a AVLTrieeNode<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            let v=self.triee.node(c).unwrap().value();
            if v>self.max_value{
                self.now=0;
            }else{
                match self.triee.next(self.now,self.same_branch){
                    Some((i,b))=>{
                        self.now=i;
                        self.same_branch=b;
                    }
                    ,_=>{
                        self.now=0;
                    }
                }
            }
            Some((self.local_index,c,&self.triee.offset(c)))
        }
    }
}
impl<'a,T:Clone+Copy+Default+std::cmp::Ord> AVLTrieeRangeIter<'a,T>{
    pub fn new(triee:&'a AVLTriee<T>,min_value:&T,max_value:&'a T)->AVLTrieeRangeIter<'a,T>{
        let (_,id)=triee.search(min_value);
        AVLTrieeRangeIter{
            now:id
            ,max_value
            ,same_branch:0
            ,local_index:0
            ,triee
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