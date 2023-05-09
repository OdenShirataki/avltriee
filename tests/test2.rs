use std::collections::HashSet;

use avltriee::Avltriee;
use avltriee::AvltrieeNode;

#[test]
fn test2() {
    let mut list: Vec<AvltrieeNode<i64>> = (0..=10).map(|_| AvltrieeNode::new(0, 0, 0)).collect();
    let rl = &mut list;
    let mut t = Avltriee::new(rl.as_mut_ptr());

    let mut deleted: HashSet<u32> = HashSet::new();

    unsafe {
        t.update_auto(1, 8);
        t.update_auto(2, 8);
        t.update_auto(3, 5);
        t.update_auto(4, 10);
        t.update_auto(5, 6);
        t.update_auto(6, 3);
        t.update_auto(7, 10);
        t.update_auto(8, 8);
        t.update_auto(9, 3);
        t.update_auto(10, 5);

        output(&t, &deleted);

        let del = vec![7, 2, 3, 3, 7, 9, 10, 8, 8, 2];
        for i in del {
            println!("delete:{}", i);
            deleted.insert(i);
            t.delete(i);
            output(&t, &deleted);
            println!("{:?}", deleted);
        }
    }

    fn output(t: &Avltriee<i64>, deleted: &HashSet<u32>) {
        let mut c = 0;
        for i in t.iter() {
            println!("{}:{}:{}", i.index(), i.row(), i.value());
            c += 1;
        }
        println!("output:{} {}\n", c, 10 - deleted.len());
    }
}
