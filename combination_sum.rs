fn comb_sum(input: &[usize], i: usize, q: &mut Vec<usize>, t: usize) {
    if i == input.len() {
        if t == 0 {
            println!("{q:?}");
        }
        return;
    }

    if t >= input[i] {
        q.push(input[i]);
        comb_sum(input, i, q, t - input[i]);
        q.pop();
    }

    comb_sum(input, i + 1, q, t);
}

fn main() {
    let mut res = vec![];
    comb_sum(vec![1, 2, 2].as_ref(), 0, &mut res, 7);
}
