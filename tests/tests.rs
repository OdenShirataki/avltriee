use std::collections::HashSet;

#[cfg(test)]
const TEST_LENGTH: u32 = 100;
const TEST_VALUE_RANGE_MIN: i64 = 0;
const TEST_VALUE_RANGE_MAX: i64 = 50;

#[test]
fn test_iter() {
    use avltriee::Avltriee;
    use avltriee::AvltrieeNode;
    use rand::distributions::{Distribution, Uniform};

    let mut list: Vec<AvltrieeNode<i64>> = (0..=TEST_LENGTH)
        .map(|_| AvltrieeNode::new(0, 0, 0))
        .collect();
    let mut t = Avltriee::new(list.as_mut_ptr());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    for i in 1..=TEST_LENGTH {
        let num = die.sample(&mut rng);
        println!("update:{}", num);
        unsafe {
            t.update(i, num);
        }
    }

    let mut deleted: HashSet<u32> = HashSet::new();
    let a = Uniform::from(1..=TEST_LENGTH);
    for _ in 1..=TEST_LENGTH {
        let i = a.sample(&mut rng);
        println!("delete:{}", i);
        deleted.insert(i);
        unsafe {
            t.delete(i);
        }
    }

    let mut c = 0;
    for i in t.iter() {
        c += 1;
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
    println!("output:{} {}", c, TEST_LENGTH as usize - deleted.len());
}
#[test]
fn test_desc_iter() {
    use avltriee::Avltriee;
    use avltriee::AvltrieeNode;
    use rand::distributions::{Distribution, Uniform};

    let mut list: Vec<AvltrieeNode<i64>> = (0..=TEST_LENGTH)
        .map(|_| AvltrieeNode::new(0, 0, 0))
        .collect();
    let mut t = Avltriee::new(list.as_mut_ptr());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    for i in 1..=TEST_LENGTH {
        let num = die.sample(&mut rng);
        println!("update:{}",i);
        unsafe {
            t.update(i, num);
        }
    }

    for i in t.desc_iter() {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
}

#[test]
fn test_iter_by_search() {
    use avltriee::Avltriee;
    use avltriee::AvltrieeNode;
    use rand::distributions::{Distribution, Uniform};

    let len = 10;
    let mut list: Vec<AvltrieeNode<i64>> = (0..=len).map(|_| AvltrieeNode::new(0, 0, 0)).collect();
    let mut t = Avltriee::new(list.as_mut_ptr());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..=20);

    for i in 1..=len {
        let num = die.sample(&mut rng);
        println!("update:{}", num);
        unsafe {
            t.update(i, num);
        }
    }

    println!("iter_by(5)");
    for i in t.iter_by(|v| v.cmp(&5)) {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
    println!("iter_range(3-5)");
    for i in t.iter_range(|v| v.cmp(&3), |v| v.cmp(&5)) {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }

    println!("iter_from(5)");
    for i in t.iter_from(|v| v.cmp(&5)) {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
    println!("iter_to(5)");
    for i in t.iter_to(|v| v.cmp(&5)) {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
}
