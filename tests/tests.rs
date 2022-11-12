use avltriee::Avltriee;
use avltriee::AvltrieeNode;

#[test]
fn test2() {
    let mut root=0;
    let mut list:Vec<AvltrieeNode<i64>>=(0..10).map(|_|AvltrieeNode::new(0,0,0)).collect();
    let rl=&mut list;
    let mut t=Avltriee::new(
        &mut root
        ,rl.as_mut_ptr()
    );
    unsafe{
        t.update(1,1);
        t.update(2,1);
        t.update(3,1);

        t.update(1,1);
        t.update(2,1);
        t.update(3,1);

        t.update(4,2);
        t.update(5,2);
        t.update(6,2);

        t.update(1,2);
        t.update(2,2);
        t.update(3,2);
        t.update(4,2);
        t.update(5,2);
        t.update(6,2);

        t.update(7,3);
        t.update(8,3);
        t.update(9,3);

        t.update(1,3);
        t.update(2,3);
        t.update(3,3);
        t.update(4,3);
        t.update(5,3);
        t.update(6,3);
        t.update(7,3);
        t.update(8,3);
        t.update(9,3);
    }
}
#[test]
fn example() {
    let mut root=0;
    let mut list:Vec<AvltrieeNode<i64>>=(0..100).map(|_|AvltrieeNode::new(0,0,0)).collect();
    let rl=&mut list;
    let mut t=Avltriee::new(
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
        unsafe{
            t.update(insert_row,i);
        }
    }

    for r in t.iter(){
        println!("{},{},{:?}",r.index(),r.row(),r.value());
    }

    let (ord,row)=t.search(&92);
    println!("{:?},{}",ord,row);

    unsafe{
        t.remove(2);
    }

    insert_row+=1;
    unsafe{
        t.update(insert_row,1);
    }

    println!("iter_range");
    for r in t.iter_by_value_from_to(&20,&30){
        println!("{},{},{:?}",r.index(),r.row(),r.value());
    }

    println!("iter_value_from");
    for r in t.iter_by_value_from(&50){
        println!("{},{},{:?}",r.index(),r.row(),r.value());
    }

    println!("iter_value_to");
    for r in t.iter_by_value_to(&90){
        println!("{},{},{:?}",r.index(),r.row(),r.value());
    }
}
