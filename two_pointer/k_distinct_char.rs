use std::collections::HashMap;

fn solve(s: &str, k: usize) -> usize {
    let s = s.chars().collect::<Vec<char>>();

    let mut hm = HashMap::new();

    let mut maxi = 0;
    let mut cnt = 0;

    let mut i = 0;
    let mut j = 0;
    while i <= j && j < s.len() {
        if hm.len() < k {
            hm.entry(&s[j]).and_modify(|v| *v += 1).or_insert(1);
            cnt += 1;
            j += 1;
            maxi = maxi.max(cnt);
        } else if hm.len() == k && hm.contains_key(&s[j]) {
            hm.entry(&s[j]).and_modify(|v| *v += 1);
            cnt += 1;
            j += 1;
            maxi = maxi.max(cnt);
        } else {
            if let Some(v_c) = hm.get(&s[i]) {
                cnt -= v_c;
            }
            hm.remove(&s[i]);
            i += 1;
        }
    }
    maxi
}

fn main() {
    assert_eq!(solve("abcabab", 3), 7);
    assert_eq!(solve("abcdefgh", 3), 3);
    assert_eq!(solve("ababababa", 1), 1);

    assert_eq!(solve("ababababa", 2), 9);
    assert_eq!(solve("abebababa", 2), 5);
}
