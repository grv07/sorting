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
    let mut input = vec![3, 2, 1, 4, 5, 1, 1, 2];
    println!("Input:  {input:?}");

    quick(&mut input, 0, 7);
    println!("Output: {input:?}");
}
