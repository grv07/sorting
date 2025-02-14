fn priority(a: char) -> i32 {
    let v = vec![
        ('(', 0),
        (')', 0),
        ('^', 1),
        ('*', 2),
        ('/', 2),
        ('+', 3),
        ('-', 3),
    ];

    for (c, p) in v {
        if c == a {
            return p;
        }
    }
    -1
}

fn solve(a: &str) -> String {
    let mut s = vec![];
    let mut ans = String::new();

    for c in a.chars() {
        if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') {
            ans.push(c);
        } else if c == '(' {
            s.push(c);
        } else if c == ')' {
            while let Some(a) = s.pop() {
                if a == '(' {
                    break;
                }
                ans.push(a);
            }
        } else {
            while !s.is_empty() && (priority(c) <= priority(*s.last().unwrap())) {
                if let Some(v) = s.pop() {
                    ans.push(v);
                }
            }
            s.push(c);
        }
    }

    while let Some(v) = s.pop() {
        ans.push(v);
    }

    ans
}

fn main() {
    let ans = solve("a+b*(p^q)");
    println!("Res: {ans}");

    let ans = solve("(a+b*(p^q))/1");
    println!("Res: {ans}");
}
