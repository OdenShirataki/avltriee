use std::collections::HashSet;

use avltriee::{Avltriee, AvltrieeUpdate};

#[test]
fn test2() {
    let mut t = Avltriee::new();

    let mut deleted: HashSet<u32> = HashSet::new();

    t.update(1.try_into().unwrap(), &8);
    t.update(2.try_into().unwrap(), &8);
    t.update(3.try_into().unwrap(), &5);
    t.update(4.try_into().unwrap(), &10);
    t.update(5.try_into().unwrap(), &6);
    t.update(6.try_into().unwrap(), &3);
    t.update(7.try_into().unwrap(), &10);
    t.update(8.try_into().unwrap(), &8);
    t.update(9.try_into().unwrap(), &3);
    t.update(10.try_into().unwrap(), &5);

    output(&t, &deleted);

    let del = vec![7, 2, 3, 3, 7, 9, 10, 8, 8, 2];
    for i in del {
        println!("delete:{}", i);
        deleted.insert(i);
        t.delete(i.try_into().unwrap());
        output(&t, &deleted);
        println!("{:?}", deleted);
    }

    fn output(t: &Avltriee<i64>, deleted: &HashSet<u32>) {
        let mut c = 0;
        for i in t.iter() {
            println!("{}:{}:{}", c, i, **unsafe { t.get_unchecked(i) });
            c += 1;
        }
        println!("output:{} {}\n", c, 10 - deleted.len());
    }
}
