fn solve(a: &str) -> Option<String> {
    let mut s = vec![];

    for c in a.chars().rev() {
        // println!("{c} {s:?}");
        if c == '*' || c == '/' || c == '+' || c == '-' || c == '^' {
            if let (Some(s1), Some(s2)) = (s.pop(), s.pop()) {
                s.push(format!("({}{}{})", s1, c, s2));
            }
        } else {
            s.push(c.to_string());
        }
    }

    s.pop()
}

fn main() {
    let a = "/*ab^cr";
    let res = solve(a).unwrap();
    println!("Res: {res:?}");
}
