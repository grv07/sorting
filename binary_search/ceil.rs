fn solve(a: &[i32], t: i32) -> i32 {
    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    let mut ans = a.len() as i32;

    while low <= high {
        let mid = low + (high - low) / 2;
        if a[mid as usize] >= t {
            ans = ans.min(mid);
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4, 5], 3), 2); // First ≥ 3 is at index 2 (value: 3)
    assert_eq!(solve(&[1, 2, 4, 6, 8], 5), 3); // First ≥ 5 is at index 3 (value: 6)
    assert_eq!(solve(&[1, 3, 5, 7], 6), 3); // First ≥ 6 is at index 3 (value: 7)
    assert_eq!(solve(&[2, 4, 6, 8], 1), 0); // First ≥ 1 is at index 0 (value: 2)
    assert_eq!(solve(&[10, 20, 30, 40], 25), 2); // First ≥ 25 is at index 2 (value: 30)
    assert_eq!(solve(&[5, 10, 15], 20), 3); // No element ≥ 20, return arr.len() (out of bounds)
    assert_eq!(solve(&[2, 2, 2, 2], 2), 0); // First ≥ 2 is at index 0 (all elements are 2)
    assert_eq!(solve(&[], 5), 0); // Empty array, always return 0
}
