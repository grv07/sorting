fn solve(a: &[i32], k: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();
    for i in a {
        hm.entry(i).and_modify(|v| *v += 1).or_insert(1);
    }

    let n = a.len();
    let mut freq = vec![vec![]; n];
    for (k, v) in hm {
        freq[v].push(k);
    }

    let mut res = vec![];
    for bucket in freq.into_iter().rev() {
        for v in bucket.into_iter() {
            res.push(*v);

            if res.len() as i32 == k {
                return res;
            }
        }
    }

    res
}

fn main() {
    let res = solve(&[1, 2, 3, 1, 2, 2, 2, 23, 3], 1);
    println!("{res:?}");
}
