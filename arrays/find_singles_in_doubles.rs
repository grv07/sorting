fn solve(input: &[i32]) -> i32 {
    let mut temp = 0;

    for i in 0..input.len() {
        temp = temp ^ input[i];
    }

    temp
}

fn main() {
    let res = solve(&[1, 2, 3, 2, 3, 1, 4]);
    println!("{res}");
}
