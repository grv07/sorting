fn sort<T: std::cmp::PartialOrd>(input: &mut [T]) {
    let mut steps = 0;

    while steps < input.len() {
        for i in 0..(input.len() - steps - 1) {
            if input[i] > input[1 + i] {
                input.swap(i, 1 + i);
            }
        }
        steps += 1;
    }
}

fn main() {
    let input: &mut [i32] = &mut [1, 7, 2, -3, 4];
    sort(input);

    println!("{input:?}");
}
