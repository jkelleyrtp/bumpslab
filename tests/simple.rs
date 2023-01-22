use bumpslab::BumpSlab;

#[test]
fn basic_works() {
    #[derive(Debug, PartialEq, Eq)]
    struct Thing(usize);

    let slab = BumpSlab::new();

    let slot = slab.push(Thing(0));
    unsafe {
        assert_eq!(&*slot.ptr(), &Thing(0));
        assert_eq!(&mut *slot.ptr_mut(), &mut Thing(0));
    }
    slab.push(Thing(1));
    slab.push(Thing(2));
    slab.push(Thing(3));

    let last = slab.push(Thing(4));

    let ptr = last.ptr() as *const Thing as usize;

    slab.remove(last);

    let next = slab.push(Thing(5));

    let next_ptr = next.ptr() as *const Thing as usize;

    assert_eq!(ptr, next_ptr);
}
