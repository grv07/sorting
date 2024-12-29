fn main() {
    let mut input = vec![12, 0, 4, 1, 2, 4, 5, 90, 67, 0, 15, 11, 10, 17];
    // let mut input = vec![0, 0, 4, 1, 2, 4, 5, 10, 11];
    // let mut input = vec![4, 1, 2, 4, 5, 10, 11];
    // let input = &mut [6, 2, 3, 4, 5];

    let n = input.len() as i32;
    quick(&mut input, 0, n - 1);

    println!("{input:?}");
}

fn _swap(input: &mut [i32], a: i32, b: i32) {
    // let temp = input[a];
    // input[a] = input[b];
    // input[b] = temp;
}

fn partition_hoares(input: &mut [i32], low: i32, high: i32) -> i32 {
    let pivot = input[low as usize];
    let mut i = low - 1;
    let mut j = high + 1;

    loop {
        i += 1;
        while input[i as usize] < pivot {
            i += 1;
        }

        j -= 1;
        while input[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        input.swap(i as usize, j as usize);
    }
}

fn partition(input: &mut [i32], low: i32, high: i32) -> i32 {
    let piviot = input[low as usize];
    let mut i = low;
    let mut j = high;

    while i < j {
        while input[i as usize] <= piviot && i <= high - 1 {
            i += 1;
        }

        while input[j as usize] > piviot && j >= low + 1 {
            j -= 1;
        }

        if i < j {
            input.swap(i as usize, j as usize);
        }
    }

    input.swap(low as usize, j as usize);

    j
}

fn quick(input: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let pi = partition_hoares(input, low, high);
        quick(input, low, pi);
        // in normal partition
        // quick(input, low, pi - 1);
        quick(input, pi + 1, high);
    }
}
