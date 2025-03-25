fn solve(a: &[i32]) -> i32 {
    if a.len() < 2 {
        return a[0];
    }

    if a[0] > a[1] {
        return a[0];
    }

    if a[a.len() - 1] > a[a.len() - 2] {
        return a[a.len() - 1];
    }

    let mut low = 1;
    let mut high = a.len() as i32 - 2;

    while low <= high {
        let mid = low + (high - low) / 2;

        let mid_v = a[mid as usize];

        if mid_v > a[mid as usize - 1] && mid_v > a[mid as usize + 1] {
            return a[mid as usize];
        }

        if a[mid as usize - 1] < mid_v {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return -1;
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4, 5, 3, 2, 1]), 5);

    assert_eq!(
        solve(&[1, 2, 3, 4, 5, 3, 2, 1, 2, 3, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        9
    );
    assert_eq!(
        solve(&[1, 2, 3, 4, 5, 3, 2, 1, 2, 3, 6, 7, 8, 9, 8, 7, 6, 5]),
        9
    );
    assert_eq!(solve(&[1]), 1);
    assert_eq!(solve(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(solve(&[1, 1, 1, 1, 1]), -1);
    assert_eq!(solve(&[1, 1, 1, 1, 1, 2, 3, 2, 1, 1, 1, 1]), 3);

    assert_eq!(solve(&[1, 2, 3, 1]), 3); // Peak element is 3
    assert_eq!(solve(&[1, 2, 1, 3, 5, 6, 4]), 6); // Peak element is 6

    // Edge Cases
    assert_eq!(solve(&[10]), 10); // Single element is a peak
    assert_eq!(solve(&[1, 2]), 2); // Last element is a peak
    assert_eq!(solve(&[2, 1]), 2); // First element is a peak

    // Cases with Multiple Peaks
    assert_eq!(solve(&[1, 3, 2, 4, 1]), 3); // Peak element is 3
    assert_eq!(solve(&[10, 20, 15, 25, 30, 10]), 30); // Peak element is 30

    // Large Array Case
    assert_eq!(
        solve(&(1..=1_000_000).chain(1..=500_000).collect::<Vec<i32>>()),
        1_000_000
    );
    // Peak element is 1_000_000
}
