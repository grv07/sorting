fn solve<T: std::cmp::PartialOrd + Copy + std::fmt::Display>(a: &[T]) -> T {
    let mut sum = 0;
    let mut item: T = a[0];
    for i in 0..(a.len() - 1) {
        if item == a[i] {
            sum += 1;
        } else {
            sum -= 1;
        }

        if sum == 0 {
            item = a[i + 1];
        }
    }

    return item;
}

fn main() {
    let res = solve(&[1, 2, 3, 4, 1, 1, 1, 1, 1, 4, 4, 4, 4, 4, 4, 4, 3]);
    println!("{res:?}");
}
