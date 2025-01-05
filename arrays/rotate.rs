fn rotate<T: Copy>(input: &mut [T], d: usize) {
    let n = input.len();
    let mut d = d % n;

    while d > 0 {
        let last = input[n - 1];

        for j in (0..(n - 1)).rev() {
            input[j + 1] = input[j];
        }
        input[0] = last;

        d -= 1;
    }
}

fn main() {
    let mut input: &mut [usize] = &mut [1, 2, 3, 4];
    rotate(&mut input, 2);
    println!("{input:?}");

    let mut input: &mut [usize] = &mut [1, 2];
    rotate(&mut input, 4);
    println!("{input:?}");

    let mut input: &mut [usize] = &mut [1, 12, 2, 3, 4];
    rotate(&mut input, 201);
    println!("{input:?}");
}
