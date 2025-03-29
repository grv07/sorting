fn solve(a: &[i32]) -> i32 {
    let mut low = 0;
    let mut high = a.len() as i32 - 1;

    let mut ans = -1;
    while low <= high {
        let mid = low + (high - low) / 2;

        if a[mid as usize] == 1 {
            ans = mid;
            high = mid - 1;
        } else {
            low = low + 1;
        }
    }

    ans
}

fn call_mat(a: &[&[i32]]) -> i32 {
    let mut maxi = 0;
    for i in a {
        let idx = solve(i);
        if idx == -1 {
            continue;
        }

        let max = i.len() as i32 - idx;
        maxi = maxi.max(max);
    }

    maxi
}

fn main() {
    assert_eq!(solve(&[0, 0, 0, 0, 1, 1, 1]), 4);
    assert_eq!(solve(&[0, 0, 0, 0]), -1);
    assert_eq!(solve(&[1, 1, 1, 1, 1]), 0);

    let a: &[&[i32]] = &[
        &[0, 0, 1, 1, 1, 1],
        &[0, 0, 0, 0, 1, 1],
        &[0, 0, 0, 0, 0, 0],
        &[1, 1, 1, 1, 1, 1],
    ];
    assert_eq!(call_mat(a), 6);

    let a: &[&[i32]] = &[
        &[0, 0, 0, 0, 0, 1],
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(call_mat(a), 1);
}
