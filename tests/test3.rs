use avltriee::Avltriee;
use avltriee::AvltrieeNode;

#[test]
fn test3() {
    let mut list: Vec<AvltrieeNode<i64>> = (0..=11).map(|_| AvltrieeNode::new(0, 0, 0)).collect();
    let rl = &mut list;
    let mut t = Avltriee::new(rl.as_mut_ptr());

    unsafe {
        t.update(1, 8);
        t.update(2, 8);
        t.update(3, 5);
        t.update(4, 10);
        t.update(5, 6);
        t.update(6, 3);
        t.update(7, 10);
        t.update(8, 8);
        t.update(9, 3);
        t.update(10, 5);
        t.update(11, 11);

        assert!(t.search_eq(|v| v.cmp(&2)) == 0);
        assert!(t.search_eq(|v| v.cmp(&3)) == 9);

        assert!(t.search_gt(|v| v.cmp(&2)) == 9);
        assert!(t.search_gt(|v| v.cmp(&3)) == 10);
        assert!(t.search_gt(|v| v.cmp(&4)) == 10);
        assert!(t.search_gt(|v| v.cmp(&5)) == 5);
        assert!(t.search_gt(|v| v.cmp(&6)) == 8);
        assert!(t.search_gt(|v| v.cmp(&7)) == 8);
        assert!(t.search_gt(|v| v.cmp(&8)) == 7);
        assert!(t.search_gt(|v| v.cmp(&9)) == 7);
        assert!(t.search_gt(|v| v.cmp(&10)) == 11);
        assert!(t.search_gt(|v| v.cmp(&11)) == 0);

        assert!(t.search_ge(|v| v.cmp(&2)) == 9);
        assert!(t.search_ge(|v| v.cmp(&3)) == 9);
        assert!(t.search_ge(|v| v.cmp(&4)) == 10);
        assert!(t.search_ge(|v| v.cmp(&5)) == 10);
        assert!(t.search_ge(|v| v.cmp(&6)) == 5);
        assert!(t.search_ge(|v| v.cmp(&7)) == 8);
        assert!(t.search_ge(|v| v.cmp(&8)) == 8);
        assert!(t.search_ge(|v| v.cmp(&9)) == 7);
        assert!(t.search_ge(|v| v.cmp(&10)) == 7);
        assert!(t.search_ge(|v| v.cmp(&11)) == 11);
        assert!(t.search_ge(|v| v.cmp(&12)) == 0);

        assert!(t.search_lt(|v| v.cmp(&2)) == 0);
        assert!(t.search_lt(|v| v.cmp(&3)) == 0);
        assert!(t.search_lt(|v| v.cmp(&4)) == 9);
        assert!(t.search_lt(|v| v.cmp(&5)) == 9);
        assert!(t.search_lt(|v| v.cmp(&6)) == 10);
        assert!(t.search_lt(|v| v.cmp(&7)) == 5);
        assert!(t.search_lt(|v| v.cmp(&8)) == 5);
        assert!(t.search_lt(|v| v.cmp(&9)) == 8);
        assert!(t.search_lt(|v| v.cmp(&10)) == 8);
        assert!(t.search_lt(|v| v.cmp(&11)) == 7);
        assert!(t.search_lt(|v| v.cmp(&12)) == 11);

        assert!(t.search_le(|v| v.cmp(&2)) == 0);
        assert!(t.search_le(|v| v.cmp(&3)) == 9);
        assert!(t.search_le(|v| v.cmp(&4)) == 9);
        assert!(t.search_le(|v| v.cmp(&5)) == 10);
        assert!(t.search_le(|v| v.cmp(&6)) == 5);
        assert!(t.search_le(|v| v.cmp(&7)) == 5);
        assert!(t.search_le(|v| v.cmp(&8)) == 8);
        assert!(t.search_le(|v| v.cmp(&9)) == 8);
        assert!(t.search_le(|v| v.cmp(&10)) == 7);
        assert!(t.search_le(|v| v.cmp(&11)) == 11);
        assert!(t.search_le(|v| v.cmp(&12)) == 11);

        assert!(
            t.search_range(|v| v.cmp(&2), |v| v.cmp(&10))
                == Some(std::ops::Range { start: 9, end: 7 })
        );
        assert!(
            t.search_range(|v| v.cmp(&3), |v| v.cmp(&8))
                == Some(std::ops::Range { start: 9, end: 8 })
        );
        assert!(
            t.search_range(|v| v.cmp(&6), |v| v.cmp(&7))
                == Some(std::ops::Range { start: 5, end: 5 })
        );
        assert!(t.search_range(|v| v.cmp(&1), |v| v.cmp(&2)) == None);
        assert!(
            t.search_range(|v| v.cmp(&11), |v| v.cmp(&11))
                == Some(std::ops::Range { start: 11, end: 11 })
        );
        assert!(
            t.search_range(|v| v.cmp(&2), |v| v.cmp(&3))
                == Some(std::ops::Range { start: 9, end: 9 })
        );
        assert!(t.search_range(|v| v.cmp(&4), |v| v.cmp(&4)) == None);
    }
}
