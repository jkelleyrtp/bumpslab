# BumpSlab: A bump allocator with reusable slots

A bumpslab is a slab that provides stable references for items inside the collection.

Normally, when a vec grows, its items will move around. Bumpslab is an arena allocator that gives you the pointer, not the key. This lets you chase down the pointer contents without the arena required to use the value. This means the pointer is guaranteed stable for the lifetime of the `Slot<>`

## When to use BumpSlab:

- you need stable references (ie driving futures without Arc<Self> for waker)
- you have only one type
- iteration is less important to you than lookups


