fn solve(a: &[i32]) -> Vec<i32> {
    let mut s = vec![];
    let mut res = vec![-1; a.len()];

    for i in (0..a.len()).rev() {
        println!("{i}");

        while !s.is_empty() && a[i] >= s[s.len() - 1] {
            s.pop();
        }

        if s.is_empty() {
            res[i] = -1;
        } else {
            res[i] = s[s.len() - 1];
        }

        s.push(a[i]);
    }

    res
}

fn main() {
    let a = &[1, 2, 3, 4];
    let res = solve(a);
    println!("{a:?}");
    println!("{res:?}");

    let a = &[6, 0, 8, 1, 3];
    let res = solve(a);
    println!("{a:?}");
    println!("{res:?}");

    let a = &[4, 12, 5, 3, 1, 2, 5, 3, 12, 4, 6];
    let res = solve(a);
    println!("{a:?}");
    println!("{res:?}");
}
