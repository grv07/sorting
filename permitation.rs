fn perm(input: &[usize], mut res: Vec<usize>, mark: &mut Vec<usize>) {
    if mark.iter().sum::<usize>() == input.len() {
        return;
    }

    for i in 0..mark.len() {
        if mark[i] == 0 {
            res.push(input[i]);

            mark[i] = 1;
            perm(input, res.clone(), mark);
            mark[i] = 0;
            res.pop();
        }
    }
}

fn main() {
    let res = vec![];
    let mut mark = vec![0; 5];
    perm(&[1, 2, 3, 5, 9], res, &mut mark);
}
