fn possible(a: &[i32], t: i32, k: i32) -> bool {
    let mut pair = 0;
    let mut pair_c = 1;

    for i in 0..a.len() {
        if pair + a[i] <= t {
            pair += a[i];
        } else {
            pair_c += 1;
            pair = a[i];
        }
    }

    if pair_c <= k {
        return true;
    }

    false
}

fn solve(a: &[i32], k: i32) -> i32 {
    if (a.len() as i32) < k {
        return -1;
    }

    let mut low = *a.iter().max().unwrap();
    let mut high = a.iter().sum::<i32>();

    while low <= high {
        let mid = low + (high - low) / 2;

        if possible(a, mid, k) {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn main() {
    assert_eq!(possible(&[1, 2, 4, 5, 6, 45], 45, 3), true);
    assert_eq!(possible(&[1, 2, 4, 5, 6, 45], 1, 6), true);
    assert_eq!(possible(&[1, 2, 4, 30, 6, 45], 45, 2), true);

    assert_eq!(solve(&[25, 46, 28, 49, 24], 4), 71);
    assert_eq!(solve(&[25, 46, 28, 49, 24], 5), 49);
    assert_eq!(solve(&[25, 46, 28, 49, 24], 6), -1);

    assert_eq!(solve(&[25, 25, 25, 25, 25], 5), 25);
    assert_eq!(solve(&[25, 25, 25, 25, 25], 2), 75);
    assert_eq!(solve(&[25, 25, 25, 25, 25], 3), 50);

    assert_eq!(solve(&[12, 34, 67, 90], 2), 113); // Minimized max pages: [12, 34, 67] | [90] -> 113
    assert_eq!(solve(&[10, 20, 30, 40], 2), 60); // Minimized max pages: [10, 20, 30] | [40] -> 60
    assert_eq!(solve(&[10, 20, 30, 40, 50], 2), 90); // Minimized max pages: [10, 20, 30] | [40, 50] -> 90
    assert_eq!(solve(&[5, 5, 5, 5], 2), 10); // Equal distribution possible: [5,5] | [5,5] -> 10
    assert_eq!(solve(&[15, 17, 20], 3), 20); // Each student gets one book, max pages = 20
    assert_eq!(solve(&[5, 10, 20, 30], 3), 30); // Minimized max pages: [5, 10] | [20] | [30] -> 30
    assert_eq!(solve(&[5, 10, 15, 20, 25], 3), 30); // [5, 10, 15] | [20] | [25] -> 35
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3), 17); // [1, 2, 3, 4, 5] | [6, 7] | [8, 9] -> 17
    assert_eq!(solve(&[100, 200, 300, 400], 2), 600); // [100, 200] | [300, 400] -> 500
    assert_eq!(solve(&[10, 20, 30, 40], 4), 40); // Each student gets one book, max pages = 40
    assert_eq!(solve(&[10, 20, 30], 4), -1); // More students than books, impossible case
}
