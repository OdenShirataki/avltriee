use tri_avltree::TriAVLTree;
use tri_avltree::TriAVLTreeNode;

#[test]
fn example(){

    let mut root=0;
    let mut list:Vec<TriAVLTreeNode<i64>>=Vec::with_capacity(100);
    let rl=&mut list;
    let mut t=TriAVLTree::new(
        &mut root
        ,rl.as_mut_ptr()
        ,0
    );

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
        t.insert(i);
    }

    for (local_index,id,data) in t.iter(){
        println!("{},{},{:?}",local_index,id,data);
    }

    let (ord,id)=t.search(&92);
    println!("{:?},{}",ord,id);

    t.remove(2);

    t.insert(1);

    for (local_index,id,data) in t.iter(){
        println!("{},{},{:?}",local_index,id,data);
    }
}