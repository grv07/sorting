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

// input: a+b*c
// ans: *+abc

// Solution:
// 1. Reverse the string
// 2. infix to postfix
// 3. reverse the ans

// a+b-c*(a^2) -> (2^a)*c-b+a
// ans     S
//         (
// 2
// 2       ^
// 2a
// )
// pop till (
// 2a^
//          *
// 2a^c
//          *-
// 2a^cb    *-+ // Point of change that we pop only less priority not equals
// 2a^cba   *-+
// 2a^cba+-*

// reverse the ans
// *-+abc^a2 -> a+b-c*a^2
fn solve(a: &str) -> String {
    // println!("{a}");
    // a.reverse();
    let mut ans = String::new();
    let mut s = vec![];

    for c in a.chars() {
        if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || (c >= 'A' && c <= 'Z') {
            ans.push(c);
        } else if c == ')' {
            s.push(c);
        } else if c == '(' {
            while let Some(v) = s.pop() {
                if v == ')' {
                    break;
                }
                ans.push(v);
            }
        } else {
            while !s.is_empty() && (priority(c) < priority(s[s.len() - 1])) {
                if let Some(v) = s.pop() {
                    ans.push(v);
                }
            }

            s.push(c);
        }
    }

    // println!("{ans}");
    while let Some(v) = s.pop() {
        ans.push(v);
    }

    ans.chars().rev().collect()
}

fn main() {
    let mut a = String::from("a+b*c/(d^3)");
    println!("Input: {a}");
    a = a.chars().rev().collect();

    let ans = solve(&a);
    println!("Ans: {ans}");
}
