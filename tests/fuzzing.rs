use bumpslab::BumpSlab;

#[test]
fn fuzzing() {
    let slab = BumpSlab::new();

    let mut values = Vec::new();
    for _ in 0..1000 {
        if rand::random() {
            for _ in 0..rand::random::<usize>() % 10 {
                let slot = slab.push(1234);
                unsafe {
                    assert_eq!(&*slot.ptr(), &1234);
                }
                values.push(slot);
            }
        } else {
            for _ in 0..rand::random::<usize>() % 10 {
                if !values.is_empty() {
                    let idx = rand::random::<usize>() % values.len();
                    unsafe {
                        assert_eq!(&*values[idx].ptr(), &1234);
                    }
                    let slot = values.remove(idx);
                    slab.remove(slot);
                }
            }
        }
    }
}
