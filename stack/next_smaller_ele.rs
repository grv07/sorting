fn solve(a: &[i32]) -> Vec<i32> {
    let mut s = vec![];
    let mut res = vec![-1; a.len()];

    for i in 0..a.len() {
        while !s.is_empty() && a[i] <= s[s.len() - 1] {
            s.pop();
        }

        if !s.is_empty() {
            res[i] = s[s.len() - 1];
            s.push(a[i]);
        } else {
            s.push(a[i]);
        }
    }

    res
}

fn main() {
    let a = &[3, 1, 2, 3, 4];
    let res = solve(a);
    println!("{res:?}");

    let a = &[4, 5, 2, 10, 8];
    let res = solve(a);
    println!("{res:?}");
}
