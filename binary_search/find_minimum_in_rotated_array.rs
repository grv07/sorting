fn solve(a: &[i32]) -> i32 {
    let mut ans = i32::MAX;

    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let mid_v = a[mid as usize];
        let low_v = a[low as usize];
        let high_v = a[high as usize];

        ans = ans.min(mid_v);

        if mid != low && low != high && mid_v == high_v && high_v == low_v {
            low += 1;
            high -= 1;
            continue;
        }

        if mid_v <= high_v {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(solve(&[5, 6, 7, 8, 1, 2, 3]), 1);
    assert_eq!(solve(&[5, 6, 7, 8, 0]), 0);

    // 1. Basic rotated cases
    assert_eq!(solve(&[3, 4, 5, 1, 2]), 1);
    assert_eq!(solve(&[4, 5, 6, 7, 0, 1, 2]), 0);

    // 2. Fully sorted (not rotated)
    assert_eq!(solve(&[1, 2, 3, 4, 5]), 1);

    // 3. Rotated at different positions
    assert_eq!(solve(&[5, 6, 7, 8, 1, 2, 3, 4]), 1);
    assert_eq!(solve(&[2, 3, 4, 5, 6, 7, 8, 1]), 1);

    // 4. Array with duplicates (worst-case binary search)
    assert_eq!(solve(&[2, 2, 2, 0, 1, 2]), 0);
    assert_eq!(solve(&[10, 10, 10, 1, 10, 10, 10]), 1);

    // 5. All elements are the same
    assert_eq!(solve(&[5, 5, 5, 5, 5]), 5);

    assert_eq!(solve(&[10, 10, 10, 1, 10, 10, 10]), 1);
    assert_eq!(solve(&[2, 2, 2, 0, 1, 2, 2]), 0);
    assert_eq!(solve(&[3, 4, 5, 1, 2]), 1);
    assert_eq!(solve(&[1, 1, 1, 1, 1]), 1);

    // ✅ Basic cases
    assert_eq!(solve(&[3, 4, 5, 1, 2]), 1); // Normal rotation
    assert_eq!(solve(&[1, 2, 3, 4, 5]), 1); // Already sorted

    // 🔄 **Rotation with duplicates**
    assert_eq!(solve(&[2, 2, 2, 0, 1, 2]), 0); // Min is between duplicates
    assert_eq!(solve(&[10, 10, 10, 1, 10, 10, 10]), 1); // Min buried in duplicates
    assert_eq!(solve(&[1, 1, 1, 1, 1]), 1); // All elements are the same

    // 🔥 **Extreme cases**
    assert_eq!(solve(&[100]), 100); // Single element
    assert_eq!(solve(&[2, 1]), 1); // Two elements swapped
    assert_eq!(solve(&[5, 6, 7, 8, 9, 10, 1, 2, 3, 4]), 1); // Large rotation

    // 🌀 **Fully reversed sorted array**
    assert_eq!(solve(&[5, 4, 3, 2, 1]), 1); // Not a valid rotated sorted array, but should still return min
}
