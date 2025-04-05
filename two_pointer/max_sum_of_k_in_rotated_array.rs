fn solve(a: &[i32], k: i32) -> i32 {
    let mut maxi = 0;
    let mut i: i32 = k;
    let mut j = 0;

    while j <= k && i >= 0 {
        let mut sum = 0;

        for idx in 0..i {
            sum += a[idx as usize];
        }

        for idx in 0..j {
            sum += a[a.len() - idx as usize - 1];
        }

        maxi = maxi.max(sum);

        // println!("{i} {j}");
        i -= 1;
        j += 1;
    }

    maxi
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 23, 3, 4, 6, 2, 3], 4), 29);
    assert_eq!(solve(&[1, 2, 3, 3, 3, 4, 6, 2, 3], 4), 15);
    assert_eq!(solve(&[1, 2, 3, 3, 3, 4, 6, 2, 3], 5), 18);
}
