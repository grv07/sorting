fn solve(a: &str) -> usize {
    let c: Vec<char> = a.chars().collect();

    let mut set = std::collections::HashSet::new();

    let mut maxi = 0;

    let mut i = 0;
    let mut j = 0;

    while i <= j && j < c.len() {
        if set.insert(c[j]) {
            j += 1;
        } else {
            set.remove(&c[i]);
            i += 1;
        }

        maxi = maxi.max(set.len());
    }

    maxi
}

fn main() {
    let res = solve("abcab");
    println!("Res: {res}");
    assert_eq!(res, 3);

    let res = solve("xxxxxx");
    println!("Res: {res}");
    assert_eq!(res, 1);

    let res = solve("xyzefx");
    println!("Res: {res}");
    assert_eq!(res, 5);

    let res = solve("x");
    println!("Res: {res}");
    assert_eq!(res, 1);

    let res = solve("");
    println!("Res: {res}");
    assert_eq!(res, 0);
}
