// Find longest common prefix of string in an array of string
// a = ["geeksforgeeks", "geeks", "geek", "geef"]
// ans =  "gee"

fn solve(a: &[Vec<char>]) -> usize {
    let mut c = a[0].len();
    for i in 1..a.len() {
        let m = c.min(a[i].len());
        c = 0;

        for m in 0..m {
            if a[i - 1][m] == a[i][m] {
                c += 1;
            } else {
                break;
            }
        }
    }

    c
}

fn main() {
    let a = &["geeksforgeeks", "geeks", "geek", "geef"];
    let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
    assert_eq!(3, solve(&a));

    let a = &["geeksforgeeks", "geeks", "geek", "geef", "goofy"];
    let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
    assert_eq!(1, solve(&a));

    let a = &["geef", "tgeofy"];
    let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
    assert_eq!(0, solve(&a));
}
