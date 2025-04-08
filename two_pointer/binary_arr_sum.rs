fn solve(a: &[i32], k: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut cnt = 0;
    let mut sum = 0;

    while i <= j && j < a.len() {
        cnt += a[j];

        while cnt > k {
            cnt -= a[i];
            i += 1;
        }

        // println!(" {j} {i} {}", j - i + 1);
        sum += j - i + 1;

        j += 1;
    }

    sum as i32
}

fn main() {
    let a = &[1, 0, 1, 0, 1];
    assert_eq!(solve(a, 2) - solve(a, 1), 4);

    let a = &[1, 0, 0, 1, 0, 1];
    assert_eq!(solve(a, 2) - solve(a, 1), 5);

    let a = &[1, 1, 0, 1, 1, 1];
    assert_eq!(solve(a, 2) - solve(a, 1), 6);

    let a = &[1, 1, 1, 1, 1, 1];
    assert_eq!(solve(a, 2) - solve(a, 1), 5);
}
