use std::collections::HashMap;

fn solve(a: &[i32], b: usize) -> i32 {
    let mut hm = HashMap::new();
    let mut cnt = 0;
    let mut maxi = 0;

    let mut i = 0;
    let mut j = 0;
    while i <= j && j < a.len() {
        if hm.len() < b {
            hm.entry(&a[j]).and_modify(|v| *v += 1).or_insert(1);
            cnt += 1;
            j += 1;
            maxi = maxi.max(cnt);
        } else if hm.len() == b && hm.contains_key(&a[j]) {
            hm.entry(&a[j]).and_modify(|v| *v += 1);
            cnt += 1;
            j += 1;
            maxi = maxi.max(cnt);
        } else {
            if let Some(v) = hm.get(&a[i]) {
                cnt -= v;
            }
            hm.remove(&a[i]);
            i += 1;
        }
    }
    maxi
}

fn main() {
    let ans = solve(&[3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 2);
    assert_eq!(ans, 5);

    let ans = solve(&[1, 1, 1, 1, 3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 8);
    assert_eq!(ans, 15);

    let ans = solve(&[1, 1, 1, 1, 3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 1);
    assert_eq!(ans, 4);

    let ans = solve(&[1, 3, 2, 1, 2, 3, 4], 8);
    assert_eq!(ans, 7);

    let ans = solve(&[1, 3, 2, 1, 2, 3, 4], 3);
    assert_eq!(ans, 6);

    let ans = solve(&[1, 3, 2, 1, 2, 3, 4], 2);
    assert_eq!(ans, 3);
}
