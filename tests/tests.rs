use std::{collections::HashSet, ops::Deref};

#[cfg(test)]
const TEST_LENGTH: u32 = 100;
const TEST_VALUE_RANGE_MIN: i64 = 0;
const TEST_VALUE_RANGE_MAX: i64 = 50;

#[test]
fn test_iter() {
    use avltriee::Avltriee;
    use rand::distributions::{Distribution, Uniform};

    let mut t = Avltriee::new();

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    futures::executor::block_on(async {
        for i in 1..=TEST_LENGTH {
            let num = die.sample(&mut rng);
            println!("update:{}", num);
            t.update(i.try_into().unwrap(), num).await;
        }
    });

    let mut deleted: HashSet<u32> = HashSet::new();
    let a = Uniform::from(1..=TEST_LENGTH);
    for _ in 1..=TEST_LENGTH {
        let i = a.sample(&mut rng);
        println!("delete:{}", i);
        deleted.insert(i);
        t.delete(i.try_into().unwrap());
    }

    let mut c = 0;
    for i in t.iter() {
        c += 1;
        println!("{}:{}:{}", c, i, unsafe { t.get_unchecked(i) }.deref());
    }
    println!("output:{} {}", c, TEST_LENGTH as usize - deleted.len());
}

#[test]
fn test_desc_iter() {
    use avltriee::Avltriee;
    use rand::distributions::{Distribution, Uniform};

    let mut t = Avltriee::new();

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    futures::executor::block_on(async {
        for i in 1..=TEST_LENGTH {
            let num = die.sample(&mut rng);
            println!("update:{}", i);
            t.update(i.try_into().unwrap(), num).await;
        }
    });

    for i in t.desc_iter() {
        println!("{}:{}", i, unsafe { t.get_unchecked(i) }.deref());
    }
}

#[test]
fn test_iter_by_search() {
    use avltriee::Avltriee;
    use rand::distributions::{Distribution, Uniform};

    let len = 10;
    let mut t = Avltriee::new();

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..=20);

    futures::executor::block_on(async {
        for i in 1..=len {
            let num = die.sample(&mut rng);
            println!("update:{}", num);
            t.update(i.try_into().unwrap(), num).await;
        }
    });

    println!("iter_by(5)");
    for i in t.iter_by(|v| v.cmp(&5)) {
        println!("{}:{}", i, unsafe { t.get_unchecked(i) }.deref());
    }
    println!("iter_range(3-5)");
    for i in t.iter_range(|v| v.cmp(&3), |v| v.cmp(&5)) {
        println!("{}:{}", i, unsafe { t.get_unchecked(i) }.deref());
    }

    println!("iter_from(5)");
    for i in t.iter_from(|v| v.cmp(&5)) {
        println!("{}:{}", i, unsafe { t.get_unchecked(i) }.deref());
    }
    println!("iter_to(5)");
    for i in t.iter_to(|v| v.cmp(&5)) {
        println!("{}:{}", i, unsafe { t.get_unchecked(i) }.deref());
    }
}

#[test]
fn test_insert_10000() {
    use avltriee::Avltriee;

    const TEST_LENGTH: u32 = 1000000;

    let mut t = Avltriee::new();

    futures::executor::block_on(async {
        for i in 1..=TEST_LENGTH {
            t.insert(i).await;
        }
    });

    println!("OK:{}", 1000000);
}
