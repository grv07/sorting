fn solve(a: &mut [i32]) {
    let mut pre = vec![1; a.len()];
    let mut post = vec![1; a.len()];

    let mut pre_pro = 1;
    let mut post_pro = 1;

    let n = a.len();

    for i in 0..n {
        pre[i] = pre_pro;
        pre_pro *= a[i];

        let j = n - (i + 1);
        post[j] = post_pro;
        post_pro *= a[j];
    }

    for i in 0..a.len() {
        a[i] = pre[i] * post[i];
    }
}

fn main() {
    let res = &mut [1, 2, 3, 4];
    solve(res);
    println!("Res: {res:?}");

    let res = &mut [-1, 1, 0, -1, 3];
    solve(res);
    println!("Res: {res:?}");

    let res = &mut [-1, 1, 1, -1, 3];
    solve(res);
    println!("Res: {res:?}");

    let res = &mut [-1, 1, 1, -1, 3, 0];
    solve(res);
    println!("Res: {res:?}");
}
