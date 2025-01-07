fn solve(input: &[i32], t: i32) -> usize {
    let mut i = 0;
    let mut j = 0;

    let mut sum = 0;
    let mut maxi = 0;

    while i <= j && j < input.len() {
        if sum < t {
            sum += input[j];
            j += 1;
        }

        if sum > t {
            sum -= input[i];
            i += 1;
        }

        if sum == t {
            // println!("sum found at {i} {j}");
            maxi = std::cmp::max(maxi, j - i);

            sum -= input[i];
            i += 1;
        }
    }

    return maxi;
}

fn main() {
    let res = solve(&[1, 1, 1, 1, 2, 2, 4, 2], 4);
    println!("{res}");

    let res = solve(&[4, 2, 1, 1, 2, 2, 1, 1, 1, 1], 3);
    println!("{res}");

    // let res = solve(&[1, 2, 3, 1, 1, 1, 1, 2, 2, 4, 2, 3, 0, 0, 0, 0, 2], 2);
    // println!("{res}");
}
