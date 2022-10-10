use avltriee::Avltriee;
use avltriee::AvltrieeNode;

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
        t.update(insert_row,i);
    }

    for r in t.iter(){
        println!("{},{},{:?}",r.index(),r.row(),r.value());
    }

    let (ord,row)=t.search(&92);
    println!("{:?},{}",ord,row);

    t.remove(2);

    insert_row+=1;
    t.update(insert_row,1);

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
