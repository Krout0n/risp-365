use std::collections::HashMap;

fn main() {
    let key = vec![1, 2, 3];
    let val = vec!["a", "b", "c"];
    let mut m1: HashMap<_, _> = key.into_iter().zip(val).collect();

    let key = vec![3, 4, 5];
    let val = vec!["a", "b", "c"];
    let mut m2: HashMap<_, _> = key.into_iter().zip(val).collect();

    m1.extend(m2.clone());
    dbg!(m1);
    dbg!(m2);
}
