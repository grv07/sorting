// the greater element that is smallest.
fn solve(a: &[i32], t: i32) -> i32 {
    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    let mut ans = -1 as i32;
    if a.is_empty() {
        return -1;
    }

    while low <= high {
        let mid = low + ((high - low) / 2);
        if a[mid as usize] <= t {
            low = mid + 1;
            ans = ans.max(mid);
        } else {
            high = mid - 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(solve(&[1, 2, 4, 6, 8], 5), 2); // 4 (index 2) is largest ≤ 5
    assert_eq!(solve(&[1, 2, 4, 6, 8], 6), 3); // 6 (index 3) is largest ≤ 6
    assert_eq!(solve(&[1, 2, 4, 6, 8], 10), 4); // 8 (index 4) is largest ≤ 10
    assert_eq!(solve(&[1, 2, 4, 6, 8], 0), -1); // No element ≤ 0 → return -1
    assert_eq!(solve(&[3, 3, 3, 3], 3), 3); // Last 3 at index 3
    assert_eq!(solve(&[], 5), -1); // Empty array → return -1
    assert_eq!(solve(&[5], 5), 0); // Single element 5 at index 0
    assert_eq!(solve(&[2, 5, 7, 10], 6), 1); // 5 (index 1) is largest ≤ 6
}
