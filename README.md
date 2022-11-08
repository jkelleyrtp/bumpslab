# BumpSlab: A bump allocator with reusable slots

[![crates.io](https://img.shields.io/crates/v/bumpslab.svg)](https://crates.io/crates/bumpslab)


A bumpslab is a slab that provides stable references for items inside the collection.

Normally, when a vec grows, its items will move around. Bumpslab is an arena allocator that gives you the pointer, not the key. This lets you chase down the pointer contents without the arena required to use the value. This means the pointer is guaranteed stable for the lifetime of the `Slot<>`

## Example

```rust
struct MyThing(usize);

let slab = BumpSlab::new();

let a = slab.push(MyThing(0));
let b = slab.push(MyThing(1));
let c = slab.push(MyThing(2));

let last = slab.push(MyThing(3));

let ptr = last.ptr();
slab.remove(last);

let new_last = slab.push(Mything(4));
let new_ptr = new_last.ptr();

// Slots get reused, known by pointer
assert_eq!(ptr, new_ptr);
```


## When to use BumpSlab:

- you need stable references (ie driving futures without Arc<Self> for waker)
- you have only one type
- iteration is less important to you than lookups


## Use as an Arc alternative:

The primary motivation for this crate was to enable spawning batches of futures without requiring Arc for the custom waker code. Wakers require a stable reference, typically guaranteed by Arc. However, if you're spawning many futures, this could come with some allocation overhead. BumpSlab gives you a stable reference for the lifetime of the waker and reuses slots as they are freed, keeping superior cache locality when compared to Arc.
