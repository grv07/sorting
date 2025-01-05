fn main() {
    let mut input = vec![12, 4, 91, 2, 4, 5, 90, 67, 0, 15, 11, 10, 17];

    println!("Input: \n{input:?}");
    let n = input.len() as i32;
    let target = n / 2;
    sort_list_till_target(&mut input, 0, n - 1, target);

    println!("{input:?}");
    println!("Output: {}", input[target as usize]);
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
            println!("{input:?} {j}");
            return j;
        }

        input.swap(i as usize, j as usize);
    }
}

fn partition_lomuto(input: &mut [i32], low: i32, high: i32) -> i32 {
    let pivot = input[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if input[j as usize] <= pivot {
            i += 1;
            input.swap(i as usize, j as usize);
        }
    }
    input.swap((i + 1) as usize, high as usize);
    println!("{input:?} {i}");

    i
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

fn sort_list_till_target(input: &mut Vec<i32>, low: i32, high: i32, target: i32) {
    if low < high {
        let pi = partition(input, low, high);
        if pi == target {
            return;
        }

        if pi > target {
            sort_list_till_target(input, low, pi - 1, target);
        }
        if pi < target {
            sort_list_till_target(input, pi + 1, high, target);
        }
    }
}
