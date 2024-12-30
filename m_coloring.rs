type Input<'a> = &'a [&'a [usize]];

fn dfs(
    input: &[&[usize]],
    s: usize,
    /*p: &mut Vec<usize>,*/ color: &mut Vec<usize>,
    d: usize,
) -> bool {
    let n = input.len();
    if d == n {
        println!("{color:?}");
        return true;
    }

    for c in 1..n {
        for i in 0..n {
            if input[s][i] == 1 && color[i] == 0 {
                if is_safe(input, s, color, c) {
                    color[i] = c;
                    if dfs(input, i, /*p,*/ color, d + 1) {
                        return true;
                    }
                    color[i] = 0;
                }
            }
        }
    }
    false
}

fn is_safe(input: Input, v: usize, color: &mut Vec<usize>, c: usize) -> bool {
    for i in 0..input.len() {
        if input[v][i] == 1 && color[i] == c {
            return false;
        }
    }
    true
}

fn main() {
    let input: &[&[usize]] = &[&[0, 1, 1, 1], &[1, 0, 1, 0], &[1, 1, 0, 1], &[1, 0, 1, 0]];
    // let input: &[&[usize]] = &[&[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1]];
    let mut color = vec![0; input.len()];
    if !dfs(input, 0, &mut color, 0) {
        println!("Not found");
    }
}
