// Input:  *a+bc
// Output: (a*(b+c))

fn solve(a: &str) -> String {
    let operators = vec!['*', '-', '+', '/', '^'];
    let mut s = vec![];

    for c in a.chars().rev() {
        if operators.iter().any(|v| *v == c) {
            if let (Some(s1), Some(s2)) = (s.pop(), s.pop()) {
                s.push(format!("({s1}{c}{s2})"));
            }
        } else {
            s.push(c.to_string());
        }
    }

    s.pop().unwrap()
}

fn main() {
    let a = "*a+bc";
    let res = solve(a);
    println!("{res}");

    let a = "^/*a+bcph";
    let res = solve(a);
    println!("{res}");
}
