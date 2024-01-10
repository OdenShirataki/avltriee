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

let mut triee = Avltriee::new();
```

### insert & update

```rust
unsafe {
    triee.insert(100).await; //or triee.update(1.try_into().unwrap(), 100).await;
}
unsafe {
    triee.insert(345).await; //or triee.update(2.try_into().unwrap(), 345).await;
}
unsafe {
    triee.insert(789).await; //or triee.update(3.try_into().unwrap(), 789).await;
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
let (ord,row) = triee.search(|v|v.cmp(&100));
if ord==Ordering::Equal{
    //found 
}
```

