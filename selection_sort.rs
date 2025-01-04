fn sort<T: std::cmp::PartialOrd>(input: &mut [T]) {
    let mut low = 0;

    while low < input.len() {
        let mut smallest = low;
        for j in low..input.len() {
            if input[smallest] > input[j] {
                smallest = j;
            }
        }

        input.swap(low, smallest);

        low += 1;
    }
}

fn main() {
    // let mut input: &mut [i32] = &mut [1, 4, 2, 3, 4];
    let mut input: &mut [i32] = &mut [1, -4, 0, 890, 02, 3, 4];

    sort(&mut input);
    println!("{input:?}");
}
