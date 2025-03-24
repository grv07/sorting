fn solve(a: &[i32]) -> i32 {
    if a.len() == 1 {
        return a[0];
    }

    if a.is_empty() || a.len() < 3 {
        return -1;
    }

    if a[0] != a[1] {
        return a[0];
    }

    if a[a.len() - 1] != a[a.len() - 2] {
        return a[a.len() - 1];
    }

    let mut low = 1;
    let mut high = a.len() - 2;

    while low <= high {
        let mid = low + (high - low) / 2;

        if a[mid - 1] != a[mid] && a[mid + 1] != a[mid] {
            return a[mid];
        }
        if mid % 2 == 0 {
            if a[mid - 1] != a[mid] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        } else {
            if a[mid + 1] != a[mid] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(solve(&[1, 1, 2, 2, 3, 4, 4, 5, 5]), 3);
    assert_eq!(solve(&[3]), 3);
    assert_eq!(solve(&[1, 1, 2, 2, 3]), 3);
    assert_eq!(solve(&[1, 2, 2, 3, 3]), 1);

    assert_eq!(solve(&[1, 1, 2]), 2); // Unique at end
    assert_eq!(solve(&[2, 3, 3]), 2); // Unique at start
    assert_eq!(solve(&[1, 1, 2, 2, 3]), 3); // Unique at end in longer array
    assert_eq!(solve(&[4, 4, 5, 5, 6, 7, 7]), 6); // Unique in middle
    assert_eq!(solve(&[10, 10, 20, 20, 30, 30, 40]), 40); // Unique at end
    assert_eq!(solve(&[-3, -3, -2, -2, -1]), -1); // Unique negative number
    assert_eq!(solve(&[0, 0, 1]), 1); // Unique positive with zero
    assert_eq!(solve(&[99]), 99); // Single element case
}

// 1,1,2,2,3,3,4,5,5,6,6; n=13
//           ^mid
// case1: if mid is even and a[mid-1] == a[mid]
// > It means single element is on rigt side
// case2: if mid is even and a[mid+1] == a[mid]
// > It means single element is on left side
