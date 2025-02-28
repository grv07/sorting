fn solve(a: &[i32]) -> usize {
    let n = a.len();
    let mut v = vec![];
    let mut pge: Vec<i32> = vec![-1; n];
    // let mut nge = vec![n as i32; n];
    // let mut res = vec![0; n];
    let mut sum = 0;

    for i in 0..a.len() {
        while let Some(last) = v.last() {
            if a[*last] < a[i] {
                // nge[*last] = i as i32;
                let pge = || -> usize {
                    if pge[*last] == -1 {
                        0
                    } else {
                        (a[i].min(a[pge[*last] as usize]) as usize - a[*last] as usize)
                            * (i - pge[*last] as usize - 1)
                    }
                };

                sum += pge();
                v.pop();
            } else {
                pge[i] = *last as i32;
                break;
            }
        }

        v.push(i);
    }

    // println!("NGE: {nge:?}");
    // println!("PGE: {pge:?}");
    // println!("WATER: {sum:?}");
    sum
}

fn main() {
    let res = solve(&[1, 2, 3, 2, 1, 4]);
    println!("RES: {res}");

    let res = solve(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
    println!("RES: {res}");

    let res = solve(&[2, 1, 0, 5, 3]);
    println!("RES: {res}");

    let res = solve(&[]);
    println!("RES: {res}");

    let res = solve(&[5]);
    println!("RES: {res}");

    let res = solve(&[5, 3]);
    println!("RES: {res}");

    let res = solve(&[5, 5, 5, 5]);
    println!("RES: {res}");

    let res = solve(&[1, 2, 3, 4]);
    println!("RES: {res}");

    let res = solve(&[4, 3, 2, 1]);
    println!("RES: {res}");

    let res = solve(&[3, 0, 3]);
    println!("RES: {res}");

    let res = solve(&[3, 0, 2, 0, 4]);
    println!("RES: {res}");

    let res = solve(&[1000, 0, 1000, 500, 1000, 0, 1000]);
    println!("RES: {res}");

    let res = solve(&[0, 1, 0, 1, 0, 1, 0]);
    println!("RES: {res}");

    let res = solve(&[0, 2, 1, 3, 0, 1, 2, 1, 2, 1]);
    println!("RES: {res}");
}
