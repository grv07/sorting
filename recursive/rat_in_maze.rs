// Rat start from (0, 0) in N*N. it has to reach the destination (N-1, N-1);
// Find all of the possible paths
// possoble movement is left, top, right, bottom
// value 0 means blocked
// value 1 means can travel

fn sol(input: &[&[isize]], s: (isize, isize), mut path: Vec<(isize, isize)>) {
    let N: isize = input.len() as isize;

    let (i, j) = s;

    if path.contains(&(i, j)) {
        return;
    }

    if i == j && i == N - 1 {
        path.push((i, j));
        print!("Reached ");
        println!("Path {path:?}");
        return;
    }

    path.push((i, j));

    for n in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let np = (i + n.0, j + n.1);
        if np.0 >= N
            || np.0 < 0
            || np.1 >= N
            || np.1 < 0
            || input[np.0 as usize][np.1 as usize] == 0
        {
            continue;
        }

        sol(input, np, path.clone());
    }
}

fn main() {
    let input: &[&[isize]] = &[&[1, 0, 0, 0], &[1, 1, 0, 1], &[1, 1, 0, 0], &[0, 1, 1, 1]];
    let path = vec![];
    sol(input, (0, 0), path);
}
