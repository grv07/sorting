fn solve(s: &str, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut left = 0;
    let mut right = 0_usize;
    let mut maxi = 0_usize;
    let mut max_count = 0;
    let mut count = [0_i32; 26];

    while left <= right && right < s.len() {
        count[s[right] as usize - 65] += 1;

        max_count = max_count.max(count[s[right] as usize - 65]);

        let gap = (right - left + 1) - max_count as usize;

        if gap <= k as usize {
            maxi = maxi.max(right - left + 1);
        } else {
            count[s[left] as usize - 65] -= 1;
            left += 1;
        }

        right += 1;
    }

    maxi as i32
}

fn main() {
    assert_eq!(solve("AABAACBBAA", 2), 6);
    assert_eq!(solve("ABAABBBAA", 2), 6);
    assert_eq!(solve("AAAAAA", 2), 6);
    assert_eq!(solve("AAAAAACCBBBBBB", 2), 8);
    assert_eq!(solve("AAAAAACCBBBBB", 2), 8);
    assert_eq!(solve("AAAAACCBBBBBB", 2), 8);
    assert_eq!(solve("ACB", 2), 3);
    assert_eq!(solve("AAACCCCBBB", 2), 6);
    assert_eq!(solve("AAACCCBBB", 2), 5);
}
