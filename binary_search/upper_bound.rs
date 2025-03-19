// The upper bound of a target x in a sorted array is the index of the first element that is strictly greater than x.

fn solve(a: &[i32], t: i32) -> i32 {
    let mut low = 0;
    let mut high = a.len() as i32 - 1;
    let mut ans = 0;

    while low <= high {
        let mid = low + ((high - low) / 2);

        if a[mid as usize] <= t {
            low = mid + 1;
            ans = mid.max(low);
        } else {
            high = mid - 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4], 5), 4);
    assert_eq!(solve(&[1, 2, 3, 3, 3, 3, 3, 4], 3), 7);

    assert_eq!(solve(&[1, 2, 3, 4, 5], 3), 3); // First element > 3 is at index 3 (value: 4)
    assert_eq!(solve(&[1, 2, 3, 3, 4, 5], 3), 4); // First element > 3 is at index 4 (value: 4)
    assert_eq!(solve(&[1, 2, 3, 4, 5], 5), 5); // No element > 5, return length (5)
    assert_eq!(solve(&[1, 2, 3, 4, 5], 0), 0); // First element > 0 is at index 0 (value: 1)

    assert_eq!(solve(&[], 3), 0); // Empty array → return 0
    assert_eq!(solve(&[10], 5), 0); // Single element array, target smaller → return 0
    assert_eq!(solve(&[10], 10), 1); // Single element array, target equal → return 1
    assert_eq!(solve(&[10], 15), 1); // Single element array, target greater → return 1

    assert_eq!(solve(&[1, 1, 2, 2, 2, 3, 3, 4], 2), 5); // First element > 2 is at index 5 (value: 3)
    assert_eq!(solve(&[1, 1, 1, 1, 1], 1), 5); // All elements = 1, return array length (5)

    assert_eq!(solve(&[-10, -5, 0, 3, 7, 10], -6), 1); // First element > -6 is at index 1 (value: -5)
    assert_eq!(solve(&[-10, -5, 0, 3, 7, 10], -10), 1); // First element > -10 is at index 1 (value: -5)
    assert_eq!(solve(&[-10, -5, 0, 3, 7, 10], 8), 5); // First element > 8 is at index 5 (value: 10)
    assert_eq!(solve(&[-10, -5, 0, 3, 7, 10], 10), 6); // No element > 10, return length (6)

    assert_eq!(solve(&(1..=10).collect::<Vec<i32>>(), 5), 5);
    assert_eq!(
        solve(&(1..=1_000_000).collect::<Vec<i32>>(), 500_000),
        500_000
    );
    assert_eq!(
        solve(&(1..=1_000_000).collect::<Vec<i32>>(), 1_000_000),
        1_000_000
    );
}
