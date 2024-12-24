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

fn perm2(input: &mut [usize], i: usize) {
    if i == input.len() {
        println!("{input:?}");
        return;
    }

    for k in i..input.len() {
        let temp = input[k];
        input[k] = input[i];
        input[i] = temp;

        perm2(input, i + 1);

        let temp = input[i];
        input[i] = input[k];
        input[k] = temp;
    }
}

fn main() {
    let res = vec![];
    let mut mark = vec![0; 5];
    perm(&[1, 2, 3, 5, 9], res, &mut mark);
    perm2(&mut [1, 2, 3], 0);
}
