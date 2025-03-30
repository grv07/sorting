fn solve(a: &[&[i32]], t: i32) -> (i32, i32) {
    let m = a.len() as i32;
    let n = a[0].len() as i32;

    let mut low = 0;
    let mut high = m * n - 1;

    //   M
    // 0 1 2
    // 3 4 5
    // 6 7 8
    //
    // 0 + 8/2 = 4
    // 4/3 = 1
    while low <= high {
        let mid = low + (high - low) / 2;
        let (r, c) = (mid as f32 / m as f32, (mid % n));

        // println!("{mid} {r} {c}");

        if a[r as usize][c as usize] == t {
            return (r as i32, c);
        }

        if a[r as usize][c as usize] < t {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    (-1, -1)
}

fn main() {
    let a: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    assert_eq!(solve(a, 9), (2, 2));

    let a: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    assert_eq!(solve(a, 1), (0, 0));

    let a: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    assert_eq!(solve(a, 7), (2, 0));

    let a: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];
    assert_eq!(solve(a, 6), (1, 2));
}
