fn is_closed(a: char) -> bool {
    if a == ')' || a == '}' || a == ']' {
        return true;
    }
    false
}

fn is_pair(a: char, b: char) -> bool {
    if a == '(' && b == ')' || a == '[' && b == ']' || a == '{' && b == '}' {
        return true;
    }
    false
}

fn solve(a: &str) -> bool {
    let mut s = vec![];
    for c in a.chars() {
        if is_closed(c) {
            if let Some(a) = s.pop() {
                if !is_pair(a, c) {
                    // println!("Not pair {a} {c}");
                    return false;
                }
            } else {
                // println!("Empty stack");
                return false;
            }
        } else {
            s.push(c);
        }
    }

    // println!("Is empty: {s:?} {}", s.is_empty());
    s.is_empty()
}

fn main() {
    let a = "(([({})]))";
    let res = solve(a);
    println!("Res: {res}");

    let a = "[()](([({})]))";
    let res = solve(a);
    println!("Res: {res}");

    let a = "))";
    let res = solve(a);
    println!("Res: {res}");

    let a = "(";
    let res = solve(a);
    println!("Res: {res}");
}
