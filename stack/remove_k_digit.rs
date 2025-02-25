fn solve(a: &[i32], k: i32) -> Vec<i32> {
    let mut v = vec![];
    let mut k = k;

    for i in 0..a.len() {
        if k <= 0 {
            v.push(a[i]);
            continue;
        }

        while !v.is_empty() && k > 0 {
            let s = v.len();
            if v[s - 1] > a[i] {
                k -= 1;
                v.pop();
                continue;
            }
            break;
        }

        v.push(a[i]);
    }

    for _i in 0..k {
        v.pop();
    }

    v
}

fn main() {
    let res = solve(&[1, 4, 3, 1, 2, 2, 9], 3);
    println!("Res: {res:?}");

    let res = solve(&[1, 1, 2, 2, 3, 3, 9], 1);
    println!("Res: {res:?}");
}
