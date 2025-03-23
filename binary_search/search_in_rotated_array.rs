fn solve(a: &[i32], t: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = a.len() as i32 - 1;

    while low <= high {
        let mid = low + ((high - low) / 2);

        if a[mid as usize] == t {
            return mid as i32;
        }

        if a[mid as usize] < a[high as usize] {
            // right is sorted
            if a[mid as usize] <= t && a[high as usize] >= t {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        } else {
            // left is sorted
            if a[low as usize] <= t && a[mid as usize] >= t {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
    }

    -1
}

fn main() {
    let res = solve(&[7, 8, 9, 1, 2, 3, 4, 5, 6], 2);
    assert_eq!(res, 4);

    let res = solve(&[7, 8, 9, 1, 2, 3, 4, 5, 6], 1);
    assert_eq!(res, 3);

    assert_eq!(solve(&[4, 5, 6, 7, 0, 1, 2], 0), 4); // Found at index 4
    assert_eq!(solve(&[4, 5, 6, 7, 0, 1, 2], 3), -1); // Not found
    assert_eq!(solve(&[1], 1), 0); // Single element, found at index 0
    assert_eq!(solve(&[1], 2), -1); // Single element, not found
    assert_eq!(solve(&[3, 1], 1), 1); // Rotated, found at index 1
    assert_eq!(solve(&[3, 1], 3), 0); // Rotated, found at index 0
    assert_eq!(solve(&[5, 6, 7, 1, 2, 3, 4], 3), 5); // Found at index 5
    assert_eq!(solve(&[5, 6, 7, 1, 2, 3, 4], 6), 1); // Found at index 1
    assert_eq!(solve(&[5, 6, 7, 1, 2, 3, 4], 8), -1); // Not found
    assert_eq!(solve(&[], 1), -1); // Empty array
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7], 4), 3); // Normal sorted array (no rotation)
    assert_eq!(solve(&[6, 7, 8, 1, 2, 3, 4, 5], 8), 2); // Found at index 2
    assert_eq!(solve(&[6, 7, 8, 1, 2, 3, 4, 5], 5), 7); // Found at index 7
}
