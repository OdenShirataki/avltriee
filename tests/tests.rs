use avltriee::AVLTriee;
use avltriee::AVLTrieeNode;

#[test]
fn example() {
    let mut root=0;
    let mut list:Vec<AVLTrieeNode<i64>>=Vec::with_capacity(100);
    let rl=&mut list;
    let mut t=AVLTriee::new(
        &mut root
        ,rl.as_mut_ptr()
    );

    let mut insert_row=0;
    let d=vec![
        110i64
        ,100
        ,90
        ,10
        ,60
        ,40
        ,20
        ,30
        ,30
        ,30
        ,50
        ,30
        ,70
        ,80
    ];
    for i in d{
        insert_row+=1;
        t.update(insert_row,i);
    }

    for (local_index,row,data) in t.iter(){
        println!("{},{},{:?}",local_index,row,data);
    }

    let (ord,row)=t.search(&92);
    println!("{:?},{}",ord,row);

    t.remove(2);

    insert_row+=1;
    t.update(insert_row,1);

    println!("iter_range");
    for (local_index,row,data) in t.iter_by_value_from_to(&20,&30){
        println!("{},{},{:?}",local_index,row,data);
    }

    println!("iter_value_from");
    for (local_index,row,data) in t.iter_by_value_from(&50){
        println!("{},{},{:?}",local_index,row,data);
    }

    println!("iter_value_to");
    for (local_index,row,data) in t.iter_by_value_to(&90){
        println!("{},{},{:?}",local_index,row,data);
    }
}
