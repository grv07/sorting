fn possible(a: &[i32], c: i32) -> i32 {
    let mut days = 1;
    let mut sum = 0;

    for i in 0..a.len() {
        if sum + a[i] <= c {
            sum += a[i];
        } else {
            days += 1;
            sum = a[i];
        }
    }

    days
}

fn solve(a: &[i32], d: i32) -> i32 {
    let mut low = *a.iter().max().unwrap();
    let mut high = a.iter().sum();
    println!("{high}");

    while low <= high {
        let mid = low + (high - low) / 2;
        println!("{mid} {low} {high} {}", possible(a, mid));

        if possible(a, mid) <= d {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn main() {
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 11], 2), 17);
    assert_eq!(solve(&[1, 2, 3, 4, 5, 10, 11, 10], 5), 11);
    assert_eq!(solve(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
}
