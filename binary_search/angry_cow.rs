fn can_place_cow(a: &[i32], cows: i32, d: i32) -> bool {
    let mut cc = 0;

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

    while i < max {
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
    assert_eq!(solve(&[0, 3, 4, 7, 9, 11], 4), 2);

    assert_eq!(solve(&[0, 3, 4, 7, 9, 11], 1), 11);

    assert_eq!(linear_solve(&[0, 3, 4, 7, 9, 11], 4), 2);

    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 1), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 2), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 3), false);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 3, 3), true);
    assert_eq!(can_place_cow(&[0, 3, 4, 7, 9, 11], 4, 4), false);
}
