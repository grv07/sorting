// Inp = [1, 2, 3, 1, 12, 0, 9, 8, 7]
// We need to find the next grtr element in circular array.
// Sol = [2, 3, 12, 12, -1, 9, 12, 9, 8]

// I solve it with two loops for first one we keep the monotonic stack
// second one will use this and keep the result.

fn solve(a: &[i32]) -> Vec<i32> {
    let mut s = vec![];
    let mut res = vec![-1; a.len()];

    let n = a.len();
    for i in 0..(2 * a.len()) {
        let i = n - (i % n) - 1;
        while !s.is_empty() && a[i] >= s[s.len() - 1] {
            s.pop();
        }

        if !s.is_empty() {
            if i <= n {
                res[i] = s[s.len() - 1];
            }
            s.push(a[i]);
        }

        s.push(a[i]);
    }

    res
}

fn main() {
    let res = solve(&[1, 2, 3, 1, 12, 0, 9, 8, 7]);
    println!("{res:?}");

    let res = solve(&[13, 2, 3, 1, 12, 0, 9, 8, 7]);
    println!("{res:?}");

    let res = solve(&[2, 10, 12, 1, 11]);
    println!("{res:?}");
}
