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

fn solve2(s: &str) -> usize {
    let s = s.as_bytes();
    let mut p = [0; 3];
    let mut l = 0;
    let mut r = 0;
    let mut cnt = 0;

    'outer: while r < s.len() - 2 {
        while p[0] == 0 || p[1] == 0 || p[2] == 0 {
            if r >= s.len() {
                break 'outer;
            }

            p[s[r] as usize - 'a' as usize] += 1;
            r += 1;
        }

        cnt += s.len() - r + 1;

        p[s[l] as usize - 'a' as usize] -= 1;

        l += 1;
    }

    cnt
}

fn main() {
    assert_eq!(solve("abcbbbb"), 5);
    assert_eq!(solve2("abcbbbb"), 5);
    assert_eq!(solve("aabcbbbb"), 10);
    assert_eq!(solve("abcabc"), 10);
    assert_eq!(solve("aaacb"), 3);
    assert_eq!(solve("acb"), 1);
    assert_eq!(solve("aca"), 0);
    assert_eq!(solve("acbbcac"), 11);
    assert_eq!(solve("abcabcabc"), 28);
}
