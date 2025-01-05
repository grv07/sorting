fn main() {
    let mut res = vec![];
    let input = &[1, 2, 3];
    target_sum(input, 0, 6, &mut res);
}

fn target_sum(input: &[i32], i: usize, t: i32, res: &mut Vec<i32>) {
    if i == input.len() {
        if t == 0 {
            println!("Target comb found! {res:?}");
        }
        return;
    }

    if t >= input[i] {
        res.push(input[i]);
        target_sum(input, i, t - input[i], res);
        res.pop();
    }

    target_sum(input, i + 1, t, res);
}
