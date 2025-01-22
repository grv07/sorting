use std::collections::HashMap;

fn solve(a: &[i32], k: i32) {
    let mut hm = HashMap::new();
    hm.insert(0, 1);
    let mut sum = 0;

    let mut maxi = 0;
    for i in 0..a.len() {
        sum = sum + a[i];

        let t = sum - k;

        if let Some(f) = hm.get(&t) {
            maxi += *f;
        }

        hm.entry(sum).and_modify(|v| *v += 1).or_insert(1);
    }

    println!("Max: {maxi}");
}

fn main() {
    solve(&[1, 2, 3, -3, 1, 1, 1, 4, 2, -3], 3);
}
