fn solve<T: PartialOrd + From<i32>>(input: &[T]) -> i32 {
    let mut max = 0;
    let mut cntr = 0;

    for i in 0..input.len() {
        if input[i] == 1.into() {
            cntr += 1;
            max = std::cmp::max::<i32>(max, cntr);
        } else {
            cntr = 0;
        }
    }

    max
}

fn main() {
    let res = solve(&[1, 2, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1]);

    println!("{res}");
}
