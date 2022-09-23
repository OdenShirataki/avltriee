use super::AVLTriee;
use super::AVLTrieeNode;

pub struct AVLTrieeIter<'a,T>{
    now:u32
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a AVLTriee<T>
}
impl<'a,T:Clone + Copy + Default> Iterator for AVLTrieeIter<'a,T> {
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
impl<'a,T:Clone + Copy + Default> AVLTrieeIter<'a,T>{
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
    ,end_row:u32
    ,same_branch:u32
    ,local_index:isize
    ,triee:&'a AVLTriee<T>
}
impl<'a,T:Clone + Copy + Default> Iterator for AVLTrieeRangeIter<'a,T> {
    type Item = (isize,u32,&'a AVLTrieeNode<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.now==0{
            None
        }else{
            self.local_index += 1;
            let c=self.now;
            if c==self.end_row{
                let same=self.triee.offset(c).same();
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
            Some((self.local_index,c,&self.triee.offset(c)))
        }
    }
}
impl<'a,T:Clone + Copy + Default> AVLTrieeRangeIter<'a,T>{
    pub fn new_with_value(triee:&'a AVLTriee<T>,value_min:&T,value_max:&'a T)->AVLTrieeRangeIter<'a,T> where T:std::cmp::Ord{
        let (_,min_row)=triee.search(value_min);
        let (_,max_row)=triee.search(value_max);
        AVLTrieeRangeIter{
            now:min_row
            ,end_row:max_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn new_with_value_max(triee:&'a AVLTriee<T>,value_max:&'a T)->AVLTrieeRangeIter<'a,T> where T:std::cmp::Ord{
        let (_,max_row)=triee.search(value_max);
        AVLTrieeRangeIter{
            now:triee.min(triee.root() as u32)
            ,end_row:max_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
    pub fn new(
        triee:&'a AVLTriee<T>,now:u32,end_row:u32
    ) -> AVLTrieeRangeIter<'a,T>
    {
        AVLTrieeRangeIter{
            now
            ,end_row
            ,same_branch:0
            ,local_index:0
            ,triee
        }
    }
}
