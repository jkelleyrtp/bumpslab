use bumpslab::BumpSlab;

#[test]
fn fuzzing() {
    let slab = BumpSlab::new();

    let mut values = Vec::new();
    for _ in 0..1000 {
        if rand::random() {
            for _ in 0..rand::random::<usize>() % 10 {
                values.push(slab.push(0));
            }
        } else {
            for _ in 0..rand::random::<usize>() % 10 {
                if let Some(slot) = values.pop() {
                    slab.remove(slot);
                }
            }
        }
    }
}
