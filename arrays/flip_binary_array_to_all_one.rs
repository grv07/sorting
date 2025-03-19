fn solve(a: &mut [i8]) -> i8 {
    let mut flips = 0;
    for i in 0..a.len() - 2 {
        if a[i] == 0 {
            flips += 1;
            for k in 0..3 {
                if a[i + k] == 1 {
                    a[i + k] = 0;
                } else {
                    a[i + k] = 1;
                }
            }
        }
    }

    println!("{a:?}");

    if a[a.len() - 1] != 1 || a[a.len() - 2] != 1 {
        return -1;
    }

    flips
}

fn main() {
    let res = solve(&mut [0, 0, 1, 1, 1]);
    println!("Res: {res}");

    let res = solve(&mut [0, 0, 1, 1, 1, 0]);
    println!("Res: {res}");

    let res = solve(&mut [0, 0, 1, 1, 1, 0]);
    println!("Res: {res}");

    let res = solve(&mut [0, 1, 1]);
    println!("Res: {res}");
}
