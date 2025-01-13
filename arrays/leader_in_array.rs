fn leader<T: std::cmp::PartialOrd + From<usize> + Copy>(a: &[T]) -> Vec<T> {
    let mut res = vec![a[a.len() - 1]];
    let mut leader: T = a[a.len() - 1];
    for i in (0..a.len()).rev() {
        // println!("{i}");
        if leader < a[i] {
            leader = a[i];
            res.push(leader);
        }
    }

    res
}

fn main() {
    let res = leader(&mut [111, 2, 3, 4, 4, 4, 45]);
    println!("{res:?}");

    let res = leader(&mut [12, 22, 10, 4, 46, 4, 45]);
    println!("{res:?}");

    let res = leader(&mut [12, 22, 10, 4, 45, 4, 45]);
    println!("{res:?}");

    let res = leader(&mut [123, 22, 10, 40, 45, 4, 45]);
    println!("{res:?}");

    let res = leader(&mut [5, 4, 3, 2, 1, 0]);
    println!("{res:?}");
}
