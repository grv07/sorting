fn min_hours(pile: i32, n: i32) -> i32 {
    let ans = pile as f32 / n as f32;
    ans.ceil() as i32
}

fn solve(a: &[i32], h: i32) -> i32 {
    let mut low = 1;
    let mut high = *a.iter().max().unwrap_or(&0);

    while low <= high {
        let mid = low + (high - low) / 2;

        let mid_v = a.into_iter().fold(0, |acc, &x| acc + min_hours(x, mid));

        if mid_v > h {
            low = mid + 1;
        } else {
            high = mid - 1;
        };
    }

    low
}

fn main() {
    let res = solve(&[3, 6, 7, 11], 8);
    assert_eq!(res, 4);

    let res = solve(&[3, 3, 3, 2], 17);
    assert_eq!(res, 1);

    let res = solve(&[3, 23, 3, 2], 7);
    assert_eq!(res, 6);

    let res = solve(&[3, 23, 3, 2], 4);
    assert_eq!(res, 23);

    assert_eq!(solve(&[3, 6, 7, 11], 8), 4); // Koko can eat all in 8 hours at speed 4
    assert_eq!(solve(&[30, 11, 23, 4, 20], 5), 30); // Needs speed 30 to finish in 5 hours
    assert_eq!(solve(&[30, 11, 23, 4, 20], 6), 23); // Needs speed 23 to finish in 6 hours
    assert_eq!(solve(&[1, 1, 1, 9999999], 10), 1428572); // Large pile case
    assert_eq!(solve(&[2, 2, 2, 2, 2], 5), 2); // Already optimal speed
    assert_eq!(solve(&[1, 1, 1, 1, 1], 5), 1); // Minimal case, needs speed 1
    assert_eq!(solve(&[1000000, 1000000], 2), 1000000); // Edge case, huge piles
    assert_eq!(solve(&[5], 10), 1); // One pile, takes 1 hour to eat all
}
