//! Compares Avltriee against a simple model (row -> value map) under random
//! insert / update / delete operations, including many duplicate values.

use std::{collections::BTreeMap, num::NonZeroU32};

use avltriee::{Avltriee, AvltrieeSearch, AvltrieeUpdate};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, n: u32) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) % n as u64) as u32
    }
}

fn check(triee: &Avltriee<u32>, model: &BTreeMap<u32, u32>) {
    for (&row, &value) in model {
        assert_eq!(
            triee.value(NonZeroU32::new(row).unwrap()),
            Some(&value),
            "row {row}"
        );
    }

    let rows: Vec<u32> = triee.iter().map(|r| r.get()).collect();
    assert_eq!(rows.len(), model.len());
    let values: Vec<u32> = rows.iter().map(|&r| model[&r]).collect();
    assert!(values.windows(2).all(|w| w[0] <= w[1]), "not sorted");

    let mut desc: Vec<u32> = triee.desc_iter().map(|r| r.get()).collect();
    desc.sort();
    let mut asc = rows.clone();
    asc.sort();
    assert_eq!(asc, desc);

    for value in [0, 5, 10, 19] {
        let expected = model.values().filter(|&&v| v == value).count();
        assert_eq!(triee.iter_by(&value).count(), expected, "iter_by {value}");
    }
}

#[test]
fn random_operations_match_model() {
    for seed in 0..20 {
        let mut rng = Lcg(seed);
        let mut triee = Avltriee::<u32>::new();
        let mut model = BTreeMap::new();

        for _ in 0..2000 {
            let value = rng.next(20);
            match rng.next(10) {
                0..=4 => {
                    let row = triee.insert(&value);
                    model.insert(row.get(), value);
                }
                5..=7 if !model.is_empty() => {
                    let keys: Vec<u32> = model.keys().copied().collect();
                    let row = keys[rng.next(keys.len() as u32) as usize];
                    triee.update(NonZeroU32::new(row).unwrap(), &value);
                    model.insert(row, value);
                }
                _ if !model.is_empty() => {
                    let keys: Vec<u32> = model.keys().copied().collect();
                    let row = keys[rng.next(keys.len() as u32) as usize];
                    triee.delete(NonZeroU32::new(row).unwrap());
                    model.remove(&row);
                }
                _ => {}
            }
            check(&triee, &model);
        }
    }
}

#[test]
fn update_row_in_duplicate_group() {
    let mut triee = Avltriee::<u32>::new();
    let r1 = triee.insert(&100);
    let r2 = triee.insert(&345);
    let r3 = triee.insert(&789);
    let r4 = triee.insert(&345);

    triee.update(r2, &1234);

    assert_eq!(triee.value(r1), Some(&100));
    assert_eq!(triee.value(r2), Some(&1234));
    assert_eq!(triee.value(r3), Some(&789));
    assert_eq!(triee.value(r4), Some(&345));
    assert_eq!(triee.iter().count(), 4);

    let r5 = triee.insert(&1);
    assert_eq!(r5.get(), 5);
}
