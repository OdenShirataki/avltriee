use std::collections::HashSet;
use std::ptr::NonNull;

use avltriee::Avltriee;
use avltriee::AvltrieeNode;

#[test]
fn test2() {
    let mut list: Vec<AvltrieeNode<i64>> = (0..=10).map(|_| AvltrieeNode::new(0, 0, 0)).collect();
    let rl = &mut list;
    let mut t = Avltriee::new(unsafe { NonNull::new_unchecked(rl.as_mut_ptr()) });

    let mut deleted: HashSet<u32> = HashSet::new();

    unsafe {
        futures::executor::block_on(async {
            t.update(1.try_into().unwrap(), 8).await;
            t.update(2.try_into().unwrap(), 8).await;
            t.update(3.try_into().unwrap(), 5).await;
            t.update(4.try_into().unwrap(), 10).await;
            t.update(5.try_into().unwrap(), 6).await;
            t.update(6.try_into().unwrap(), 3).await;
            t.update(7.try_into().unwrap(), 10).await;
            t.update(8.try_into().unwrap(), 8).await;
            t.update(9.try_into().unwrap(), 3).await;
            t.update(10.try_into().unwrap(), 5).await;
        });

        output(&t, &deleted);

        let del = vec![7, 2, 3, 3, 7, 9, 10, 8, 8, 2];
        for i in del {
            println!("delete:{}", i);
            deleted.insert(i);
            t.delete(i.try_into().unwrap());
            output(&t, &deleted);
            println!("{:?}", deleted);
        }
    }

    fn output(t: &Avltriee<i64>, deleted: &HashSet<u32>) {
        let mut c = 0;
        for i in t.iter() {
            println!("{}:{}:{}", c, i, unsafe { t.value_unchecked(i) });
            c += 1;
        }
        println!("output:{} {}\n", c, 10 - deleted.len());
    }
}
