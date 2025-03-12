// continues increasing sequence
// input: [3,1,2,3,5,4,7], out: 4 since length of [1,2,3,5]

fn solve(a: &[i32]) -> i32 {
    let mut maxi = 1;
    let n = a.len();

    if n == 0 {
        return 0;
    }

    let mut i = 1;

    let mut cnt = 1;
    while i < n - 1 {
        // if no more continue increasing sequence
        if a[i] >= a[i + 1] {
            cnt = 0;
        }

        cnt += 1;
        maxi = maxi.max(cnt);
        i += 1;
    }

    maxi
}

fn main() {
    let res = solve(&[3, 1, 2, 3, 5, 4, 7, 2, 3, 4, 5, 6]);
    println!("Res: {res}");

    let res = solve(&[2, 2, 2, 2]);
    println!("Res: {res}");

    let res = solve(&[2]);
    println!("Res: {res}");

    let res = solve(&[]);
    println!("Res: {res}");
}
