fn solve(a: &str, b: &str) -> bool {
    println!("a: {a}, b: {b}");
    let mut i = 0 as i32;
    let mut j = (a.len() - 1) as i32;

    let mut rc = 0;
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    while i < a.len() as i32 && j >= 0 {
        if a[i as usize] == b[i as usize] {
            i += 1;
            continue;
        }

        if a[j as usize] == b[j as usize] {
            j -= 1;
            continue;
        }

        if a[i as usize] == b[j as usize] {
            rc += 1;
            i += 1;
            j -= 1;
        } else {
            // println!("else {} {}", a[i as usize], b[j as usize]);
            return false;
        }
    }

    if rc > 0 && rc != 2 {
        return false;
    }

    true
}

fn solve_clean(a: &str, b: &str) -> bool {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut res = vec![];
    for i in 0..a.len() {
        if a[i] != b[i] {
            res.push(i);
        }

        if res.len() > 2 {
            return false;
        }
    }

    if res.len() == 0 {
        return true;
    }

    if let (Some(i), Some(j)) = (res.pop(), res.pop()) {
        if a[i] == b[j] && b[i] == a[j] {
            return true;
        }
    }

    false
}

fn main() {
    let sol = solve("abcd", "abcd");
    println!("{sol}");
    let sol = solve_clean("abcd", "abcd");
    println!("{sol}");

    let sol = solve("abcd", "abdc");
    println!("{sol}");
    let sol = solve_clean("abcd", "abdc");
    println!("{sol}");

    let sol = solve("abcd", "badc");
    println!("{sol}");
    let sol = solve_clean("abcd", "badc");
    println!("{sol}");

    let sol = solve("abcd", "abcp");
    println!("{sol}");
    let sol = solve_clean("abcd", "abcp");
    println!("{sol}");

    let sol = solve("abcdii", "abdcii");
    println!("{sol}");
    let sol = solve_clean("abcdii", "bacdii");
    println!("{sol}");

    let sol = solve("abcdii", "bacdii");
    println!("{sol}");
    let sol = solve_clean("abcdii", "bacdii");
    println!("{sol}");
}
