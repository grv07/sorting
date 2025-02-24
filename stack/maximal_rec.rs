fn dump(a: &mut [&mut [i32]]) {
    for r in a {
        println!("{r:?}");
    }
}

fn maximal_rec(a: &[i32]) -> i32 {
    let n = a.len();
    let mut nse = vec![n as i32; n];
    let mut pse = vec![-1 as i32; n];
    let mut v = vec![];

    let mut maxi = 0;

    for i in 0..a.len() {
        while let Some(last) = v.last() {
            if a[*last] as i32 >= a[i] {
                nse[*last] = i as i32;
                maxi = maxi.max((i as i32 - pse[*last as usize] - 1) * a[*last]);
                v.pop();
            } else {
                pse[i] = *last as i32;
                break;
            }
        }
        v.push(i);
    }

    while let Some(last) = v.pop() {
        maxi = maxi.max((n as i32 - pse[last] - 1) * a[last]);
    }

    // println!("INP: {a:?}");
    // // println!("PSE: {pse:?}");
    // // println!("NSE: {nse:?}");
    // println!("STA: {v:?}");
    // println!("MAX: {maxi:?}");

    maxi
}

fn solve(a: &mut [&mut [i32]]) -> i32 {
    let mut maxi = 0;
    // calculate the suffix sum of column
    // (sum + a[i][j]) * a[i][j]

    for i in 0..a[0].len() {
        let mut sum = 0;
        for j in 0..a.len() {
            sum += a[j][i];
            a[j][i] = sum * a[j][i];
        }
    }

    dump(a);

    for a in a {
        maxi = maxi.max(maximal_rec(a));
    }

    maxi
}

fn main() {
    let a: &mut [&mut [i32]] = &mut [
        &mut [1, 0, 1, 0, 1],
        &mut [1, 0, 1, 1, 1],
        &mut [1, 1, 1, 1, 1],
        &mut [1, 0, 0, 1, 0],
    ];

    let res = solve(a);
    println!("Res: {res}");

    // maximal_rec(&[1, 0, 1, 0, 1]);
}
