fn solve(a: &[i32]) {
    let mut v = vec![(a[0], i32::MAX)];
    let n = a.len();
    let mut i = 1;

    while i < n {
        // println!("{v:?}");
        if let Some((value, ub)) = v.last() {
            if a[i] < *ub {
                let ub = if a[i] < *value { value } else { ub };
                v.push((a[i], *ub));
            } else {
                v.pop();
                continue;
            }
        }
        i += 1;
    }
}

fn main() {
    let a = &[8, 5, 1, 7, 10, 12];
    solve(a);
}
