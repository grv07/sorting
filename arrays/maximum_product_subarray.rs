// [2, 3, -4, 5, 2, -1, -2]
fn solve(a: &[i32]) -> i32 {
    let n = a.len();

    let mut pre = a[0];
    let mut suf = a[n - 1];
    let mut maxi = i32::MIN;

    for i in 1..n - 1 {
        pre = pre * a[i];

        let p = a[n - i - 1];
        suf = suf * p;

        maxi = maxi.max(pre.max(suf));
    }

    pre *= a[n - 1];
    suf *= a[0];

    maxi.max(pre.max(suf))
}

fn main() {
    let res = solve(&[2, 3, -4, 5, 2, -1, -25]);
    println!("Ans: {res}");

    let res = solve(&[-2, 3, -4, 5, 2, -1, -25]);
    println!("Ans: {res}");
}
