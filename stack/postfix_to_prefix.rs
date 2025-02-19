fn solve(a: &str) -> String {
    let mut s = vec![];
    let mut ans = String::new();

    let op = vec!['=', '-', '+', '*', '^', '/'];

    for c in a.chars() {
        if op.iter().any(|v| *v == c) {
            if let (Some(s1), Some(s2)) = (s.pop(), s.pop()) {
                let aa = format!("{c}{s2}{s1}");
                s.push(aa);
            }
        } else {
            s.push(c.to_string());
        }
    }

    while let Some(aa) = s.pop() {
        ans.push_str(&aa);
    }

    ans
}

fn main() {
    let res = solve("ca-b*p=");
    // println!("{res}");

    assert_eq!(solve("AB+"), "+AB");
    assert_eq!(solve("ABC*+"), "+A*BC");
    assert_eq!(solve("AB+C-"), "-+ABC");
    assert_eq!(solve("AB*CD+/"), "/*AB+CD");
    assert_eq!(solve("AB+C*D/"), "/*+ABCD");
    assert_eq!(solve("AB+CD+*"), "*+AB+CD");
    assert_eq!(solve("ABC-*"), "*A-BC");
    assert_eq!(solve("ABC*DE/-+"), "+A-*BC/DE");
    assert_eq!(solve("A"), "A");
}
