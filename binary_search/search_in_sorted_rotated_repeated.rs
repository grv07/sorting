fn solve(a: &[i32], t: i32) -> bool {
    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if a[mid as usize] == t {
            return true;
        }

        if a[mid as usize] == a[low as usize] && a[low as usize] == a[high as usize] {
            low += 1;
            high -= 1;
            continue;
        }

        if a[mid as usize] <= a[high as usize] {
            if a[mid as usize] <= t && a[high as usize] >= t {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        } else {
            if a[mid as usize] >= t && a[low as usize] <= t {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
    }

    false
}

fn main() {
    assert_eq!(solve(&[2, 5, 6, 0, 0, 1, 2], 0), true); // Target exists
    assert_eq!(solve(&[2, 5, 6, 0, 0, 1, 2], 3), false); // Target not in array
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1, 2, 1, 1], 2), true); // Target exists despite duplicates
    assert_eq!(solve(&[3, 1, 2, 3, 3, 3, 3], 2), true); // Found in rotated part
    assert_eq!(solve(&[3, 3, 3, 3, 3, 3, 3], 4), false); // Target doesn't exist
    assert_eq!(solve(&[], 5), false); // Edge case: empty array
    assert_eq!(solve(&[1], 1), true); // Single-element match
    assert_eq!(solve(&[1], 0), false); // Single-element no match
    assert_eq!(solve(&[1, 1, 3, 1], 3), true); // Handles duplicates properly
    assert_eq!(solve(&[4, 5, 6, 7, 0, 1, 2], 0), true); // Found in second half
    assert_eq!(solve(&[4, 5, 6, 7, 0, 1, 2], 3), false); // Not found in rotated array
    assert_eq!(solve(&[10, 10, 10, 10, 1, 2, 10], 2), true); // Found in rotated portion with duplicates

    // 1. All elements are the same, target exists
    assert_eq!(solve(&[5, 5, 5, 5, 5], 5), true);

    // 2. All elements are the same, target does not exist
    assert_eq!(solve(&[5, 5, 5, 5, 5], 3), false);

    // 3. Rotated at different positions
    assert_eq!(solve(&[3, 4, 5, 1, 2], 1), true); // Found in rotated part
    assert_eq!(solve(&[3, 4, 5, 1, 2], 6), false); // Not in the array

    // 4. Rotation happens at the very end
    assert_eq!(solve(&[2, 2, 2, 3, 4, 2], 3), true); // Found
    assert_eq!(solve(&[2, 2, 2, 3, 4, 2], 1), false); // Not found

    // 5. Only two elements, rotated
    assert_eq!(solve(&[2, 1], 1), true); // Found in rotated position
    assert_eq!(solve(&[2, 1], 3), false); // Not in array

    // 6. Target is at the rotation pivot
    assert_eq!(solve(&[10, 20, 30, 5, 7, 8], 5), true);

    // 7. Single element, does not match
    assert_eq!(solve(&[42], 0), false);

    // 8. Single element, matches
    assert_eq!(solve(&[42], 42), true);

    // 9. Large array, rotated at different places
    let large_sorted = (1..=1_000_000).collect::<Vec<i32>>();
    let mut large_rotated = large_sorted.clone();
    large_rotated.rotate_right(500_000);
    assert_eq!(solve(&large_rotated, 750_000), true); // Found
    assert_eq!(solve(&large_rotated, 2_000_000), false); // Not found

    // 10. Array contains extreme values
    assert_eq!(
        solve(&[-1_000_000, -500, 0, 1, 500, 1_000_000], 1_000_000),
        true
    );
    assert_eq!(
        solve(&[-1_000_000, -500, 0, 1, 500, 1_000_000], 2_000_000),
        false
    );
}
