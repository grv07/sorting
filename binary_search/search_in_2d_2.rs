fn solve(a: &[&[i32]], t: i32) -> (i32, i32) {
    let m = a.len() as i32 - 1;
    let n = a[0].len() as i32 - 1;

    let mut r = 0 as i32;
    let mut c = n as i32;
    while r >= 0 && r <= m && c >= 0 && c <= n {
        let mid = a[r as usize][c as usize];
        // println!("mid: {mid}");

        if mid == t {
            return (r, c);
        }

        if mid > t {
            c = c - 1;
        } else {
            r = r + 1;
        }
    }

    (-1, -1)
}

fn main() {
    let a: &[&[i32]] = &[
        &[1, 4, 7, 11, 15],
        &[2, 5, 8, 12, 19],
        &[3, 6, 9, 16, 22],
        &[18, 21, 23, 26, 30],
    ];

    assert_eq!(solve(a, 15), (0, 4));

    assert_eq!(solve(a, 6), (2, 1));

    assert_eq!(solve(a, 23), (3, 2));
    assert_eq!(solve(a, 30), (3, 4));
    assert_eq!(solve(a, 18), (3, 0));
    assert_eq!(solve(a, 19), (1, 4));
}
