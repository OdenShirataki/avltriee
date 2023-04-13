use avltriee::Avltriee;
use avltriee::AvltrieeNode;

#[test]
fn test2() {
    let mut root = 0;
    let mut list: Vec<AvltrieeNode<i64>> = (0..9).map(|_| AvltrieeNode::new(0, 0, 0)).collect();
    let rl = &mut list;
    let mut t = Avltriee::new(&mut root, rl.as_mut_ptr());
    unsafe {
        t.update(1, 0);
        t.update(2, 1);
        t.update(3, 0);
        t.update(4, 1);
        t.update(5, 0);
        t.update(6, 2);
        t.update(7, 3);
        t.update(8, 4);
    }
    for i in t.iter() {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
    println!("-");
    for i in t.desc_iter() {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
}