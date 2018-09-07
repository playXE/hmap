# hmap

This crate provides simple hashmap creation, example:

```rust
#[macro_use]extern crate hmap;

fn main() {
    //generates HashMap<&'static str,i32>
    let hash_map = hmap!("one" => 1,"two" => 2);
}
```