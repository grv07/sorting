fn solve(a: &[i32], k: i32) -> usize {
    let mut maxi = 0;

    let mut i = 0;
    let mut j = 0;

    let mut limit = k;

    while i <= j && j < a.len() {
        if a[j] == 1 {
            j += 1;
        } else {
            if limit > 0 {
                limit -= 1;
                j += 1;
            } else {
                while limit <= 0 {
                    if a[i] == 0 {
                        limit += 1;
                    }
                    i += 1;
                }
            }
        }
        // println!("limit: {limit} {i} {j}");
        maxi = maxi.max(j as i32 - i as i32);
    }

    maxi as usize
}

fn main() {
    assert_eq!(solve(&[1, 1, 1, 0, 0, 1], 2), 6);
    assert_eq!(solve(&[1, 1, 0, 0, 0, 1], 2), 4);
    assert_eq!(solve(&[1, 0, 0, 0, 0,], 2), 3);
    assert_eq!(solve(&[1, 0, 0, 0, 0, 1, 1], 2), 4);
    assert_eq!(solve(&[1, 0, 0, 0, 0, 1, 1, 0, 0], 2), 4);
    assert_eq!(solve(&[0, 0, 0, 0, 0, 0], 2), 2);
    assert_eq!(solve(&[1, 1, 1, 1, 1, 1], 2), 6);
}
