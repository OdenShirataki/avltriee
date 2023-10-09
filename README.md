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
    triee.update(1.try_into().unwrap(), 100).await; //insert
}
unsafe {
    triee.update(2.try_into().unwrap(), 345).await; //insert
}
unsafe {
    triee.update(3.try_into().unwrap(), 789).await; //insert
}
unsafe {
    triee.update(2.try_into().unwrap(), 1234).await; //update exists row
}
```
### iterator

```rust
for i in triee.iter() {
    println!("{}:{}", i, unsafe{ t.value_unchecked(i) });
}
for i in triee.desc_iter() {
    println!("{}:{}", i, unsafe{ t.value_unchecked(i) });
}
for i in t.iter_from(|v|v.cmp(&10)) {
    println!("{}:{}", i, unsafe{ t.value_unchecked(i) });
}
for i in t.iter_to(|v|v.cmp(&500)) {
    println!("{}:{}", i, unsafe{ t.value_unchecked(i) });
}
for i in t.iter_range(|v|v.cmp(&300),|v|v.cmp(&999)) {
    println!("{}:{}", i, unsafe{ t.value_unchecked(i) });
}
```
### delete
```rust
triee.delete(1.try_into().unwrap());
```

### search
```rust
let (ord,row) = triee.search_end(|v|v.cmp(&100));
if ord==Ordering::Equal{
    //found 
}
```

