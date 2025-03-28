fn possible(a: &[i32], m: i32, k: i32, time: i32) -> bool {
    let mut cnt = 0;
    let mut b_cnt = 0;

    if (a.len() as i32) < (m * k) {
        return false;
    }

    let max = *a.iter().max().unwrap();
    if time == max {
        b_cnt += (a.len() as i32) / k;
        if b_cnt >= m {
            return true;
        }
    }

    for i in 0..a.len() {
        if (a[i] - time) <= 0 {
            cnt += 1;
            if cnt == k {
                b_cnt += cnt / k;
                cnt = 0;
            }
        } else {
            cnt = 0;
        }

        if b_cnt >= m {
            return true;
        }
    }

    false
}

fn linear_solve(a: &[i32], m: i32, k: i32) -> i32 {
    let mut low = *a.iter().min().unwrap();
    let high = *a.iter().max().unwrap();

    while low <= high {
        if possible(a, m, k, low) {
            return low;
        }
        low += 1;
    }

    high
}

fn solve(a: &[i32], m: i32, k: i32) -> i32 {
    let mut low = *a.iter().min().unwrap();
    let mut high = *a.iter().max().unwrap();

    let mut ans = high;
    while low <= high {
        let mid = low + (high - low) / 2;

        // println!("mid {mid} {low} {high}");
        if possible(a, m, k, mid) {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn main() {
    assert_eq!(possible(&[7, 7, 7, 7, 9, 11, 12, 7], 3, 3, 7), false);
    assert_eq!(possible(&[7, 7, 7, 7, 9, 11, 12, 7], 2, 3, 11), true);
    assert_eq!(possible(&[7, 7, 7, 7, 9, 11, 12, 7], 2, 3, 11), true);
    assert_eq!(possible(&[7, 7, 7, 7, 9, 11, 12, 7, 12], 3, 3, 12), true);
    assert_eq!(
        possible(&[7, 7, 7, 7, 9, 11, 17, 12, 7, 12], 3, 3, 12),
        true
    );

    assert_eq!(solve(&[7, 7, 7, 7, 9, 11, 12, 7, 12], 3, 3), 12);

    assert_eq!(linear_solve(&[7, 7, 7, 7, 9, 9, 17, 12, 11, 11], 3, 3), 12);
    assert_eq!(solve(&[7, 7, 7, 7, 9, 9, 19, 11, 12, 11, 17,], 3, 3), 12);
    assert_eq!(
        solve(
            &[7, 7, 7, 7, 9, 9, 11, 12, 11, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 27],
            2,
            3
        ),
        9
    );

    assert_eq!(solve(&[1, 10, 3, 10, 2], 3, 1), 3); // Need 3 bouquets of 1 flower each
                                                    // assert_eq!(solve(&[1, 10, 3, 10, 2], 3, 2), -1); // Not enough flowers for 3 bouquets of 2
    assert_eq!(solve(&[7, 7, 7, 7, 13, 11, 12, 7], 2, 3), 12); // Bouquets can be formed at day 7
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6], 2, 3), 6); // Bouquets can be made by grouping flowers
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6], 3, 2), 6); // Smallest day possible
    assert_eq!(solve(&[100, 200, 300, 400], 1, 2), 200); // Only one bouquet of 2 possible
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1, 1], 2, 3), 1); // All flowers bloom on day 1
    assert_eq!(solve(&[1, 2, 3, 100, 200, 300], 2, 2), 100); // Large gap in bloom times
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1], 3, 2), 1); // Exact number of flowers for bouquets

    // Edge case: Only 1 flower and it is enough
    assert_eq!(solve(&[5], 1, 1), 5);

    // Edge case: Single flower but not enough
    assert_eq!(solve(&[5], 2, 1), -1);

    // Large numbers edge case
    assert_eq!(solve(&[100000, 1000000, 10000000], 1, 2), 1000000);
}
