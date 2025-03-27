fn solve(word: &str, abbr: &str) -> bool {
    let word: Vec<char> = word.chars().collect();
    let abbr: Vec<char> = abbr.chars().collect();
    let mut i = 0;
    let mut k = 0;

    while i < word.len() && k < abbr.len() {
        if abbr[k].is_numeric() {
            let mut s = String::new();
            while k < abbr.len() && abbr[k].is_numeric() {
                s.push(abbr[k]);
                k += 1;
            }

            if s.starts_with('0') {
                return false;
            }

            if let Ok(s) = s.parse::<usize>() {
                i += s;
            }
        } else {
            if let (Some(kc), Some(ic)) = (abbr.get(k), word.get(i)) {
                // println!("kc: {} ic: {}", kc, ic);
                if kc != ic {
                    return false;
                }
            }
            k += 1;
            i += 1;
        }
    }

    i == word.len() && k == abbr.len()
}

fn main() {
    assert_eq!(solve("apple", "a2le"), true);
    assert_eq!(solve("appllei", "ap2le"), false);
    assert_eq!(solve("applle", "ap2le"), true);

    assert_eq!(solve("internationalization", "i12iz4n"), true); // Valid abbreviation
    assert_eq!(solve("apple", "a2e"), false); // Skipping 'ppl' is invalid
    assert_eq!(solve("substitution", "s10n"), true); // Correct abbreviation
    assert_eq!(solve("substitution", "s55n"), false); // Number too large
    assert_eq!(solve("substitution", "s010n"), false); // Leading zeros not allowed
    assert_eq!(solve("word", "w1r1"), true); // Valid abbreviation
    assert_eq!(solve("word", "w2d"), true); // Correctly skipping 'or'
    assert_eq!(solve("word", "4"), true); // Entire word abbreviated
    assert_eq!(solve("word", "5"), false); // Length mismatch
    assert_eq!(solve("word", "w0ord"), false); // '0' should not be used
    assert_eq!(solve("word", "wor"), false); // Not fully abbreviated
    assert_eq!(solve("a", "1"), true); // Single letter abbreviation
    assert_eq!(solve("a", "2"), false); // Length mismatch
    assert_eq!(solve("abbreviation", "a10n"), true); // Skipping 'bbreviation'
    assert_eq!(solve("abbreviation", "a11n"), false); // Number too large
    assert_eq!(solve("abbreviation", "a9i1"), false); // Skipping 'bbreviatio'
    assert_eq!(solve("abbreviation", "a9i2"), false); // Length mismatch
}
