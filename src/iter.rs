use super::Avltriee;
use super::AvltrieeNode;

pub struct AvlTrieeIterResult<'a,T>{
    index:isize
    ,row:u32
    ,node:&'a AvltrieeNode<T>
}
impl<'a,T> AvlTrieeIterResult<'a,T>{
    pub fn index(&self)->isize{
        self.index
    }
    pub fn row(&self)->u32{
        self.row
    }
    pub fn value(&self)->&'a T{
        self.node.value()
    }
}
pub struct AvltrieeIter<'a,T>{
    now:u32
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a Avltriee<T>
}

impl<'a,T:Clone + Copy + Default> Iterator for AvltrieeIter<'a,T> {
    type Item = AvlTrieeIterResult<'a,T>;
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
            Some(AvlTrieeIterResult{
                index:self.local_index
                ,row:c
                ,node:&self.triee.offset(c)
            })
        }
    }
}
impl<'a,T:Clone + Copy + Default>AvltrieeIter<'a,T>{
    pub fn new(triee:&'a Avltriee<T>)->AvltrieeIter<'a,T>{
        AvltrieeIter{
            now:triee.min(triee.root() as u32)
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn begin_at(triee:&'a Avltriee<T>,begin:u32)->AvltrieeIter<'a,T>{
        AvltrieeIter{
            now:begin
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
}

pub struct AvltrieeRangeIter<'a,T>{
    now:u32
    ,end_row:u32
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a Avltriee<T>
}
impl<'a,T:Clone + Copy + Default> Iterator for AvltrieeRangeIter<'a,T> {
    type Item = AvlTrieeIterResult<'a,T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            if c==self.end_row{
                let same=self.triee.offset(c).same;
                if same!=0{
                    self.end_row=same;
                }
                self.now=same;
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
            Some(AvlTrieeIterResult{
                index:self.local_index
                ,row:c
                ,node:&self.triee.offset(c)
            })
        }
    }
}
impl<'a,T:Clone + Copy + Default> AvltrieeRangeIter<'a,T>{
    pub fn new_with_value(triee:&'a Avltriee<T>,value_min:&T,value_max:&'a T)->AvltrieeRangeIter<'a,T> where T:std::cmp::Ord{
        let (_,min_row)=triee.search(value_min);
        let (_,max_row)=triee.search(value_max);
        AvltrieeRangeIter{
            now:min_row
            ,end_row:max_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn new_with_value_max(triee:&'a Avltriee<T>,value_max:&'a T)->AvltrieeRangeIter<'a,T> where T:std::cmp::Ord{
        let (_,max_row)=triee.search(value_max);
        AvltrieeRangeIter{
            now:triee.min(triee.root() as u32)
            ,end_row:max_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn new(
        triee:&'a Avltriee<T>,now:u32,end_row:u32
    ) -> AvltrieeRangeIter<'a,T>
    {
        AvltrieeRangeIter{
            now
            ,end_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
}
