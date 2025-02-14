//
fn solve(a: &str) -> String {
    let op = vec!['+', '-', '*', '/', '^'];
    let mut s = vec![];

    for c in a.chars() {
        if op.iter().any(|v| *v == c) {
            if let (Some(s1), Some(s2)) = (s.pop(), s.pop()) {
                s.push(format!("({s2}{c}{s1})"));
            }
        } else {
            s.push(c.to_string());
        }
    }

    s.pop().unwrap()
}

fn main() {
    let a = "cab+*";
    let res = solve(a);
    println!("{res}");

    let a = "ab-de+f*/";
    let res = solve(a);
    println!("{res}");
}
