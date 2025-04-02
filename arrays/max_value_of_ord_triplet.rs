fn solve(a: &[i32]) -> i32 {
    let mut i = i32::MAX;
    let mut j = i32::MIN;
    let mut k = i32::MIN;

    for p in 0..a.len() {
        i = i.min(a[p]);
        if k < a[p] {
            j = k;
            k = a[p];
        }
        if a[p] < k && a[p] > j {
            j = a[p];
        }
    }

    println!("{i} {j} {k}");

    (k - i) * j
}

fn main() {
    assert_eq!(solve(&[27, 17, 2, 3, 24, 7, 6, 5, 11, 27, 72]), 1890);
    assert_eq!(solve(&[7, 2, 3, 0, 6, 5, 11]), 77);
}
