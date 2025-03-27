fn can_place_cow(a: &[i32], cows: i32, d: i32) -> bool {
    let mut cc = 1;

    let mut prev = a[0];
    for i in 1..a.len() {
        if cc >= cows {
            break;
        }
        if (a[i] - prev) >= d {
            cc += 1;
            prev = a[i];
        }
    }

    if cc < cows {
        return false;
    }

    true
}

fn linear_solve(a: &[i32], cows: i32) -> i32 {
    let min = a[0];
    let max = a[a.len() - 1];

    let mut i = min;

    while i <= max {
        if !can_place_cow(a, cows, i) {
            return i - 1;
        }
        i += 1;
    }

    i
}

fn solve(a: &[i32], cows: i32) -> i32 {
    let mut low = 1;
    let mut high = a[a.len() - 1];

    while low <= high {
        let mid = low + (high - low) / 2;

        if can_place_cow(a, cows, mid) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    high
}

fn main() {
    assert_eq!(solve(&[0, 3, 4, 7, 9, 11], 4), 3);
    assert_eq!(solve(&[0, 3, 4, 7, 9, 11], 1), 11);

    assert_eq!(linear_solve(&[0, 3, 4, 7, 9, 11], 4), 3);

    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 1), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 2), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 3), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 3, 3), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 4), false);

    assert_eq!(solve(&[1, 2, 4, 8, 9], 3), 3); // Cows placed at 1, 4, 8
    assert_eq!(solve(&[1, 2, 4, 8, 9], 2), 8); // Cows placed at 1, 9
    assert_eq!(solve(&[1, 3, 5, 7, 9], 3), 4); // Cows placed at 1, 5, 9
    assert_eq!(solve(&[10, 20, 30, 40, 50], 3), 20); // Cows placed at 10, 30, 50
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3); // Evenly spread
    assert_eq!(solve(&[5, 10, 15, 20, 25, 30], 3), 10); // Maximum spread
    assert_eq!(solve(&[1, 100], 2), 99); // Only two positions available
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1, 1], 2), 0); // All positions are the same
    assert_eq!(solve(&[1, 2, 4, 8, 16, 32, 64, 128], 4), 31); // Exponential spacing
    assert_eq!(solve(&[1, 3, 3, 3, 6, 7, 9], 3), 3); // Some duplicates

    assert_eq!(linear_solve(&[1, 2, 4, 8, 9], 3), 3); // Cows placed at 1, 4, 8
    assert_eq!(linear_solve(&[1, 2, 4, 8, 9], 2), 8); // Cows placed at 1, 9
    assert_eq!(linear_solve(&[1, 3, 5, 7, 9], 3), 4); // Cows placed at 1, 5, 9
    assert_eq!(linear_solve(&[10, 20, 30, 40, 50], 3), 20); // Cows placed at 10, 30, 50
    assert_eq!(linear_solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3); // Evenly spread
    assert_eq!(linear_solve(&[5, 10, 15, 20, 25, 30], 3), 10); // Maximum spread
    assert_eq!(linear_solve(&[1, 100], 2), 99); // Only two positions available
    assert_eq!(linear_solve(&[1, 1, 1, 1, 1, 1, 1], 2), 0); // All positions are the same
    assert_eq!(linear_solve(&[1, 2, 4, 8, 16, 32, 64, 128], 4), 31); // Exponential spacing
    assert_eq!(linear_solve(&[1, 3, 3, 3, 6, 7, 9], 3), 3); // Some duplicates
}
