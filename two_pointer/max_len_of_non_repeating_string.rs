fn solve(s: &str) -> i32 {
    let s = s.chars().collect::<Vec<char>>();

    let mut maxi = 0;
    let vs: &mut [i32; 26] = &mut [0; 26];

    let mut i = 0;
    let mut j = 0;

    while i <= j && j < s.len() {
        vs[s[j] as usize - 'a' as usize] += 1;

        while vs[s[j] as usize - 'a' as usize] == 2 {
            vs[s[i] as usize - 'a' as usize] = 1;
            i += 1;
        }

        maxi = maxi.max(j - i);

        j += 1;
    }

    maxi as i32 + 1
}

fn main() {
    assert_eq!(solve("abcabcde"), 5);
    assert_eq!(solve("aabcabcddee"), 4);
    assert_eq!(solve("aabcabccddee"), 3);
    assert_eq!(solve("aababccddeefghi"), 5);
    assert_eq!(solve("aa"), 1);
}
