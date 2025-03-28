// Find the k'th missing number from sorted array of integers
fn get_missing_presum(a: &[i32]) -> Vec<i32> {
    let mut ps = vec![];

    let mut lm = 0;
    for i in 0..a.len() {
        if a[i] != (i as i32 + 1) {
            lm = a[i] - (i as i32 + 1);
        }
        ps.push(lm);
    }

    ps
}

fn solve(a: &[i32], k: i32) -> i32 {
    if a[0] > k {
        return k;
    }

    let ps = get_missing_presum(a);
    let mut low = 0;
    let mut high = ps.len() as i32 - 1;

    let mut ans = high;
    while low <= high {
        let mid = low + (high - low) / 2;

        if ps[mid as usize] >= k {
            high = mid - 1;
            ans = ans.min(mid);
        } else {
            low = mid + 1;
        }
    }
    // Res: [0, 0, 0, 0, 1, 1, 2, 3, 8]

    if a[ans as usize] >= k {
        a[ans as usize - 1] + (k - ps[ans as usize - 1])
    } else {
        a[ans as usize] + k - ps[ans as usize]
    }
}

fn linear_solve(a: &[i32], k: i32) -> i32 {
    if a[0] > k {
        return k;
    }

    let ps = get_missing_presum(a);
    // println!("Res: {ps:?}");

    for i in 0..ps.len() {
        if ps[i] >= k {
            let m = k - ps[i - 1];
            return a[i - 1] + m;
        }
    }

    a[a.len() - 1] + (k - ps[ps.len() - 1])
}

fn main() {
    assert_eq!(linear_solve(&[1, 2, 3, 4, 6, 7, 9, 11, 17], 5), 13);
    assert_eq!(linear_solve(&[1, 2, 3, 4, 6, 7, 9, 11, 17], 11), 20);
    assert_eq!(linear_solve(&[1, 2, 3, 4, 6, 7, 9, 11, 17], 0), 0);
    assert_eq!(linear_solve(&[1, 2, 3, 4, 6, 7, 9, 11, 17], 1), 5);
    assert_eq!(linear_solve(&[7, 9, 11, 17], 5), 5);
    assert_eq!(linear_solve(&[7, 9, 11, 17], 7), 8);

    assert_eq!(solve(&[1, 2, 3, 4, 6, 7, 9, 11, 17], 1), 5);
    assert_eq!(solve(&[1, 2, 3, 5, 6, 7, 9, 11, 17], 1), 4);
    assert_eq!(solve(&[7, 9, 11, 17], 7), 8);
    assert_eq!(solve(&[7, 9, 11, 17], 9), 12);
    assert_eq!(solve(&[7, 9, 11, 17], 11), 14);
    assert_eq!(solve(&[7, 9, 11, 17], 18), 22);
    assert_eq!(solve(&[7, 9, 11, 17], 13), 16);
}
