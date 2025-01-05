fn solve<T: std::cmp::PartialOrd + From<usize>>(input: &mut [T]) {
    let mut i = 0;
    let mut j = input.len() - 1;
    while i < j && j > 0 {
        while input[j] == 0.into() && j > 0 {
            j -= 1;
        }

        while input[i] != 0.into() && i < j {
            i += 1;
        }

        input.swap(i, j);
    }
}

fn main() {
    let input: &mut [usize] = &mut [0, 0, 0, 1, 23, 4, 0, 9, 9, 0, 8, 0, 1, 0, 0, 0, 0];
    solve::<usize>(input);
    println!("{input:?}");

    let input: &mut [usize] = &mut [1, 2, 3, 4];
    solve::<usize>(input);
    println!("{input:?}");

    let input: &mut [usize] = &mut [0, 0, 0, 0];
    solve::<usize>(input);
    println!("{input:?}");
}
