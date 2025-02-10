use std::collections::HashMap;
// a pair is bad if i > j && j-i != a[j]-a[i]
// so to solve this we find all the good pair and minus all the pairs
// think them (i, a[i]), (j, a[j]) as slop in graph

fn solve(a: &[i32]) -> i32 {
    let mut g_p = 0;
    let mut t_p = 0;

    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 0..a.len() {
        t_p += 1;
        let k = a[i] - i as i32;

        if let Some(v) = map.get_mut(&k) {
            g_p += *v;
            *v += 1;
        } else {
            map.insert(k, 1);
        }
    }

    // println!("{map:?}");

    t_p - g_p
}

fn main() {
    let res = solve(&[4, 1, 3, 3]);
    println!("{res:?}");

    let res = solve(&[4, 1, 3, 3, 4, 6, 8]);
    println!("{res:?}");
}
