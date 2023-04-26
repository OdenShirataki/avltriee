# avltriee
## Features
A customized version of AVLTree.
Process the same value in the third branch.
This allows efficient searches even for sets with small cardinality in the distribution of values.

One data is immovable from one row, and positional relationships such as left, right, and parent are all referenced by row numbers.
No search is required for value reference by specifying a row.


## Example

### init
```rust
use avltriee::Avltriee;
use avltriee::AvltrieeNode;

let length=100;

let mut buffer: Vec<AvltrieeNode<i64>> = (0..=length)
    .map(|_| AvltrieeNode::new(0, 0, 0))
    .collect();
let mut triee = Avltriee::new(buffer.as_mut_ptr());
```

### insert & update

```rust
unsafe {
    triee.update(1, 100); //insert
}
unsafe {
    triee.update(2, 345); //insert
}
unsafe {
    triee.update(3, 789); //insert
}
unsafe {
    triee.update(2, 1234); //update exists row
}
```
### iterator

```rust
for i in triee.iter() {
    println!("{}:{}:{}", i.index(), i.row(), i.value());
}
for i in triee.desc_iter() {
    println!("{}:{}:{}", i.index(), i.row(), i.value());
}
for i in t.iter_by_value_from(10) {
    println!("{}:{}:{}", i.index(), i.row(), i.value());
}
for i in t.iter_by_value_to(500) {
    println!("{}:{}:{}", i.index(), i.row(), i.value());
}
for i in t.iter_by_value_from_to(300,999) {
    println!("{}:{}:{}", i.index(), i.row(), i.value());
}
```
### remove
```rust
triee.remove(1);
```

### search
```rust
let (ord,row) = triee.search(100);

triee.search_cb(|s| -> Ordering { /*hoge.cmp(s)*/ })
```

