# avltriee
## Features
A customized version of AVLTree.
Process the same value in the third branch.
This allows efficient searches even for sets with small cardinality in the distribution of values.

One data is immovable from one row, and positional relationships such as left, right, and parent are all referenced by row numbers.
No search is required for value reference by specifying a row.

## Example

```rust
use std::num::NonZeroU32;

use avltriee::{Avltriee, AvltrieeSearch, AvltrieeUpdate};

let mut triee = Avltriee::<u32>::new();

// insert returns the row of the new value.
let row1 = triee.insert(&100);
let row2 = triee.insert(&345);
let row3 = triee.insert(&789);
triee.insert(&345); // same values are kept in the third branch

// update an existing row.
triee.update(row2, &1234);

// get a value by row.
assert_eq!(triee.value(row1), Some(&100));
assert_eq!(triee.value(NonZeroU32::new(99).unwrap()), None);

// iterate rows in value order.
for row in triee.iter() {
    println!("{}:{}", row, triee.value(row).unwrap());
}
for row in triee.desc_iter() {
    println!("{}:{}", row, triee.value(row).unwrap());
}
let values: Vec<u32> = triee
    .iter_range(&300, &999)
    .map(|row| *triee.value(row).unwrap())
    .collect();
assert_eq!(values, [345, 789]);

// iter_from, iter_to, iter_over, iter_under and iter_by are also available.
assert_eq!(triee.iter_from(&345).count(), 3);
assert_eq!(triee.iter_by(&345).count(), 1);

// search the row of a value.
assert_eq!(triee.row(&789), Some(row3));
assert_eq!(triee.row(&1), None);

// delete a row.
triee.delete(row1);
assert_eq!(triee.value(row1), None);
```
