fn solve(a: &[i32]) -> i32 {
    let mut res = 0;

    let mut low: i32 = 0;
    let mut high: i32 = a.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_v = a[mid as usize];
        let low_v = a[low as usize];
        let high_v = a[high as usize];

        if mid != high && high != low && mid_v == high_v && high_v == low_v {
            high -= 1;
            continue;
        }

        if a[res as usize] >= mid_v {
            res = mid;
        }

        if mid_v <= high_v {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    res
}

fn main() {
    // ✅ Basic cases
    assert_eq!(solve(&[5, 5, 6, 7, 1, 2, 3, 4]), 4);
    assert_eq!(solve(&[5, 5, 5, 5, 5, 6, 7, 1, 2, 3, 4, 4, 4, 4, 5]), 7);
    assert_eq!(solve(&[3, 4, 5, 1, 2]), 3); // Rotated 3 times
    assert_eq!(solve(&[1, 2, 3, 4, 5]), 0); // No rotation (already sorted)
    assert_eq!(solve(&[2, 3, 4, 5, 1]), 4); // Rotated 4 times

    // 🔄 **Rotation with duplicates**
    assert_eq!(solve(&[10, 10, 10, 1, 10, 10, 10]), 3); // Min at index 3
    assert_eq!(solve(&[1, 1, 1, 1, 1]), 0); // All elements are the same, no rotation
    assert_eq!(solve(&[2, 2, 2, 0, 1, 2]), 3); // Min at index 3

    // 🔥 **Extreme cases**
    assert_eq!(solve(&[100]), 0); // Single element, no rotation
    assert_eq!(solve(&[2, 1]), 1); // Rotated once (two elements swapped)
    assert_eq!(solve(&[5, 6, 7, 8, 9, 10, 1, 2, 3, 4]), 6); // Large rotation

    // 🌀 **Fully reversed sorted array**
    assert_eq!(solve(&[5, 4, 3, 2, 1]), 4); // Min at index 4 (rotated 4 times)

    // ✅ Already sorted (zero rotation)
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 0);
    assert_eq!(solve(&[100, 200, 300, 400, 500]), 0);

    // 🔄 **Minimal rotations**
    assert_eq!(solve(&[2, 1]), 1); // Rotated once
    assert_eq!(solve(&[3, 1, 2]), 1); // Rotated once (short array)

    // 🔁 **Halfway rotated**
    assert_eq!(solve(&[4, 5, 6, 1, 2, 3]), 3); // Split in the middle

    // 🔥 **Duplicates everywhere**
    assert_eq!(solve(&[2, 2, 2, 2, 2, 2, 1, 2, 2, 2]), 6); // Minimum at index 6
    assert_eq!(solve(&[10, 10, 10, 10, 10, 10, 10, 10]), 0); // All same, no rotation

    // 🚀 **Extreme cases**
    assert_eq!(solve(&[1]), 0); // Single element
    assert_eq!(solve(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9); // Fully reversed (rotated n-1 times)

    // 🌀 **Multiple rotations with varying sizes**
    assert_eq!(solve(&[7, 8, 9, 10, 1, 2, 3, 4, 5, 6]), 4);
    assert_eq!(solve(&[15, 18, 2, 3, 6, 12]), 2);

    // ⚠️ **Smallest element appearing multiple times at the boundary**
    assert_eq!(solve(&[2, 3, 4, 5, 1, 1, 1]), 4); // Min starts at index 4 but repeated
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1, 1]), 0); // All same, no rotation
}
