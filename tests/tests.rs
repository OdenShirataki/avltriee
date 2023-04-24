#[cfg(test)]
const TEST_LENGTH: u32 = 1000;
const TEST_VALUE_RANGE_MIN: i64 = 0;
const TEST_VALUE_RANGE_MAX: i64 = 500;

#[test]
fn test_iter() {
    use avltriee::Avltriee;
    use avltriee::AvltrieeNode;
    use rand::distributions::{Distribution, Uniform};

    let mut root = 0;
    let mut list: Vec<AvltrieeNode<i64>> = (0..TEST_LENGTH)
        .map(|_| AvltrieeNode::new(0, 0, 0))
        .collect();
    let rl = &mut list;
    let mut t = Avltriee::new(&mut root, rl.as_mut_ptr());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    for i in 1..TEST_LENGTH {
        let num = die.sample(&mut rng);
        unsafe {
            t.update(i, num);
        }
    }

    for i in t.iter() {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
}
#[test]
fn test_desc_iter() {
    use avltriee::Avltriee;
    use avltriee::AvltrieeNode;
    use rand::distributions::{Distribution, Uniform};

    let mut root = 0;
    let mut list: Vec<AvltrieeNode<i64>> = (0..TEST_LENGTH)
        .map(|_| AvltrieeNode::new(0, 0, 0))
        .collect();
    let rl = &mut list;
    let mut t = Avltriee::new(&mut root, rl.as_mut_ptr());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(TEST_VALUE_RANGE_MIN..=TEST_VALUE_RANGE_MAX);

    for i in 1..TEST_LENGTH {
        let num = die.sample(&mut rng);
        unsafe {
            t.update(i, num);
        }
    }

    for i in t.desc_iter() {
        println!("{}:{}:{}", i.index(), i.row(), i.value());
    }
}
