// The lower bound of a target value x in a sorted array is the index of the first element that is greater than or equal to x.

fn solve(a: &[i32], t: i32) -> i32 {
    if a.is_empty() {
        return 0;
    }

    let mut ans = a.len() as i32;

    let mut low = 0_i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        let mid = low + ((high - low) / 2);

        if a[mid as usize] >= t {
            high = mid - 1;
            ans = ans.min(mid);
        } else {
            low = mid + 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4, 5], 3), 2); // Target exists
    assert_eq!(solve(&[1, 2, 3, 4, 5], 1), 0); // First element is the lower bound
    assert_eq!(solve(&[1, 2, 3, 4, 5], 5), 4); // Last element is the lower bound

    // 🔹 Empty Array
    assert_eq!(solve(&[], 10), 0); // Always returns 0 for empty array

    // 🔹 Single Element Array
    assert_eq!(solve(&[10], 10), 0); // Target matches the only element
    assert_eq!(solve(&[10], 5), 0); // Target is smaller than the only element
    assert_eq!(solve(&[10], 15), 1); // Target is greater than the only element

    // 🔹 Even Length Array
    assert_eq!(solve(&[1, 3, 5, 7], 3), 1); // Found at index 1
    assert_eq!(solve(&[1, 3, 5, 7], 4), 2); // First element ≥ 4 is 5 at index 2

    // 🔹 Odd Length Array
    assert_eq!(solve(&[1, 3, 5, 7, 9], 6), 3); // First element ≥ 6 is 7 at index 3
    assert_eq!(solve(&[1, 3, 5, 7, 9], 8), 4); // First element ≥ 8 is 9 at index 4

    // 🔹 Target Smaller Than All Elements
    assert_eq!(solve(&[5, 10, 15], 1), 0); // No smaller elements, return first index

    // 🔹 Target Greater Than All Elements
    assert_eq!(solve(&[5, 10, 15], 20), 3); // No elements ≥ 20, return array length

    // 🔹 Repeated Elements (Duplicates)
    assert_eq!(solve(&[1, 2, 2, 2, 3], 2), 1); // First ≥ 2 is at index 1
    assert_eq!(solve(&[1, 2, 2, 2, 3], 3), 4); // First ≥ 3 is at index 4

    // 🔹 Large Array (Stress Test)
}
