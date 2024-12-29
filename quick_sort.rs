fn _partition_hoares(input: &mut [i32], low: i32, high: i32) -> i32 {
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

fn _partition_lomuto(input: &mut [i32], low: i32, high: i32) -> i32 {
    let pivot = input[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if input[j as usize] <= pivot {
            i += 1;
            input.swap(i as usize, j as usize);
        }
    }
    input.swap((i + 1) as usize, high as usize);

    i
}

fn part(input: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = input[low];

    let mut j = high;
    let mut i = low;
    while j > i {
        while input[i] <= pivot && i <= high - 1 {
            i += 1;
        }

        while input[j] > pivot && j >= low + 1 {
            j -= 1;
        }

        if i < j {
            let t = input.remove(i);
            input.insert(j, t);
        }
    }

    let temp = input[j];
    input[j] = pivot;
    input[low] = temp;

    return j;
}

fn quick(input: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi = part(input, low, high);
        quick(input, low, pi - 1);
        quick(input, pi + 1, high);
    }
}

fn main() {
    let mut _input = vec![3, 2, 1, 4, 5, 1, 1, 2];
    let mut input = vec![12, 0, 4, 1, 2, 4, 5, 90, 67, 0, 15, 11, 10, 17];
    let n = input.len();

    quick(&mut input, 0, n - 1);
    println!("Output: {input:?}");
}
