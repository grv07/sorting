fn solve(a: &[i32], b: &[i32]) -> i32 {
    // a = [1,2,3,4,5]
    // b = [2,3,6,7,14]
    // app1: Merge both of the arrays in one array with or wothout extra space then add the middle two
    // app2: Iterate smaller one till n elements
    let n = a.len();

    let mut i = 0;
    let mut j = 0;
    let mut m1 = i32::MAX;
    let mut m2 = i32::MAX;

    while (i + j) <= n {
        if i == n {
            m2 = m1;
            m1 = b[j];
            break;
        }

        if j == n {
            m2 = m1;
            m1 = a[i];
            break;
        }

        if a[i] <= b[j] {
            m2 = m1;
            m1 = a[i];
            i += 1;
        } else {
            m2 = m1;
            m1 = b[j];
            j += 1;
        }
        println!("{m2} {m1}");
    }

    println!("{m2} {m1}");
    m1 + m2
}

fn main() {
    let a = &[1, 2, 3];
    let b = &[2, 3, 4];
    // 1, 2, 2, 3, 3, 4
    let res = solve(a, b);
    println!("Res: {res}");

    let a = &[1, 2, 3, 4, 5];
    let b = &[2, 3, 6, 7, 14];
    // 1, 2, 2, 3, 3, 4, 5, 6, 7, 14
    let res = solve(a, b);
    println!("Res: {res}");

    let a = &[1, 2, 4, 6, 10];
    let b = &[4, 5, 6, 9, 12];
    // 1, 2, 2, 3, 3, 4, 5, 6, 7, 14
    let res = solve(a, b);
    println!("Res: {res}");

    let a = &[1, 12, 15, 26, 38];
    let b = &[2, 13, 17, 30, 45];
    // 1, 2, 2, 3, 3, 4, 5, 6, 7, 14
    let res = solve(a, b);
    println!("Res: {res}");

    let a = &[1, 2, 3, 4];
    let b = &[5, 6, 7, 8];
    // 1, 2, 3, 4, 5, 6, 7, 8
    let res = solve(a, b);
    println!("Res: {res}");

    let a = &[1];
    let b = &[1];
    // 1, 2, 3, 4, 5, 6, 7, 8
    let res = solve(a, b);
    println!("Res: {res}");
}
