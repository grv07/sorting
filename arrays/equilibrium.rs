fn solve(a: &[i32]) -> i32 {
    let mut fsum = 0;
    for i in a {
        fsum += i;
    }

    let mut rsum = a[0];
    for i in 1..a.len() {
        let lsum = fsum - (rsum + a[i]);
        if rsum == lsum {
            return (i) as i32;
        }
        rsum += a[i];
    }

    -1
}

fn main() {
    assert_eq!(2, solve(&mut [1, 2, 3, 3]));
    assert_eq!(2, solve(&mut [0, 3, 3, 3]));
    assert_eq!(1, solve(&mut [0, 0, 0, 0]));
    assert_eq!(-1, solve(&mut [0, 1, 2, 3]));
    assert_eq!(3, solve(&mut [0, 1, 2, 80, 3]));
}
