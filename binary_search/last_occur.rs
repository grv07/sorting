fn solve(a: &[i32], t: i32) -> i32 {
    let mut ans = -1;

    if a.is_empty() {
        return ans;
    }

    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        let mid = low + ((high - low) / 2);
        if a[mid as usize] == t {
            // low = mid + 1;
            ans = mid;
        }

        if a[mid as usize] <= t {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4, 5, 5, 5, 6, 6], 5), 6);

    // AI generated test cases
    assert_eq!(solve(&[1, 2, 4, 4, 4, 5, 6], 4), 4); // Last occurrence of 4 at index 4
    assert_eq!(solve(&[1, 2, 4, 4, 4, 5, 6], 5), 5); // Last occurrence of 5 at index 5
    assert_eq!(solve(&[1, 2, 4, 4, 4, 5, 6], 1), 0); // Last occurrence of 1 at index 0
    assert_eq!(solve(&[1, 2, 4, 4, 4, 5, 6], 6), 6); // Last occurrence of 6 at index 6
    assert_eq!(solve(&[1, 2, 4, 4, 4, 5, 6], 3), -1); // 3 is not in the array
    assert_eq!(solve(&[], 3), -1); // Empty array case
    assert_eq!(solve(&[1, 1, 1, 1, 1], 1), 4); // All elements are the target, last at index 4
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 9), 8); // Target at the end
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 10), -1); // Target greater than all elements
    assert_eq!(solve(&[2, 2, 2, 2, 3, 3, 3, 3], 3), 7); // Multiple occurrences, last at index 7
}
