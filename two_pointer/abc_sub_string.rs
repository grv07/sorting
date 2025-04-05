fn solve(s: &str) -> usize {
    let a = s.as_bytes();
    let mut p: [i32; 3] = [-1; 3];

    let mut i = 0;

    let mut sum = 0;
    while i < a.len() {
        p[a[i] as usize - 'a' as usize] = i as i32;

        if p[0 as usize] >= 0 && p[1 as usize] >= 0 && p[2 as usize] >= 0 {
            let mut min = p[0 as usize].min(p[1 as usize]);
            min = min.min(p[2 as usize]);
            sum += (min as usize) + 1;
        }

        i += 1;
    }

    sum
}

fn main() {
    assert_eq!(solve("abcbbbb"), 5);
    assert_eq!(solve("aabcbbbb"), 10);
    assert_eq!(solve("abcabc"), 10);
    assert_eq!(solve("aaacb"), 3);
    assert_eq!(solve("acb"), 1);
    assert_eq!(solve("aca"), 0);
    assert_eq!(solve("acbbcac"), 11);
    assert_eq!(solve("abcabcabc"), 28);
}
