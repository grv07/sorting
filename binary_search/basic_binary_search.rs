// This is a implimentation for basic binary search.
fn solve(a: &[i32], t: i32) -> i32 {
    if a.is_empty() {
        return 0;
    }

    let mut low: i32 = 0;
    let mut high: i32 = a.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if a[mid as usize] == t {
            return mid;
        }

        if a[mid as usize] > t {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    low + (high - low) / 2
}

fn main() {
    let res = solve(&[1], 2);
    assert_eq!(res, 1);

    let res = solve(&[], 2);
    assert_eq!(res, 0);

    let res = solve(&[1, 2, 3, 4, 5, 6], 7);
    assert_eq!(res, 6);

    let res = solve(&[1, 2, 3, 4, 5, 6], 6);
    assert_eq!(res, 5);

    let res = solve(&[1, 2, 3, 4, 5, 6], 1);
    assert_eq!(res, 0);

    let res = solve(&[1, 2, 3, 4, 5, 6], 2);
    assert_eq!(res, 1);

    let res = solve(&[1, 2, 3, 4, 5, 6], 2);
    assert_eq!(res, 1);

    let res = solve(&[1], 1);
    assert_eq!(res, 0);

    // 🔹 Empty Array
    assert_eq!(solve(&[], 10), 0); // No elements in array

    // 🔹 Single Element Array
    assert_eq!(solve(&[10], 10), 0); // Target exists
    assert_eq!(solve(&[10], 5), 0); // Target does not exist

    // 🔹 Even Length Array
    assert_eq!(solve(&[1, 3, 5, 7], 3), 1); // Found at even index
    assert_eq!(solve(&[1, 3, 5, 7], 4), 2); // Not present

    // 🔹 Odd Length Array
    assert_eq!(solve(&[1, 3, 5, 7, 9], 7), 3); // Found at odd index
    assert_eq!(solve(&[1, 3, 5, 7, 9], 6), 3); // Not present

    // 🔹 Target Smaller Than All Elements
    assert_eq!(solve(&[5, 10, 15], 1), 0); // All elements are greater

    // 🔹 Target Greater Than All Elements
    assert_eq!(solve(&[5, 10, 15], 20), 3); // All elements are smaller

    // 🔹 Large Array (Stress Test)
    let large_array: Vec<i32> = (0..1000000).collect();
    assert_eq!(solve(&large_array, 999999), 999999); // Searching last element
    assert_eq!(solve(&large_array, 500000), 500000); // Searching middle element
    assert_eq!(solve(&large_array, -1), 0); // Searching for non-existent element
}
