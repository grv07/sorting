fn solve_easy<T: std::cmp::PartialOrd + Copy + From<i32>>(a: &mut [T]) -> Vec<T> {
    let n = a.len();
    let mut e = 0;
    let mut o = 1;

    let mut res: Vec<T> = vec![0.into(); n];

    for i in 0..n {
        // println!("{e} {o} {}", a[i]);

        if a[i] >= 0.into() {
            res[e] = a[i];
            e += 2;
        }

        if a[i] < 0.into() && o < n {
            res[o] = a[i];
            o += 2;
        }
    }
    res
}

fn solve(a: &mut [i32]) {
    let n = a.len() as i32;
    let mut left = 0 as i32;
    let mut right = n - 1;

    while left < right {
        while left < right
            && ((left % 2 == 0 && a[left as usize] >= 0) || (left % 2 != 0 && a[left as usize] < 0))
        {
            left += 1;
        }

        while right > left
            && ((right % 2 == 0 && a[right as usize] > 0)
                || (right % 2 != 0 && a[right as usize] < 0))
        {
            right -= 1;
        }

        a.swap(left as usize, right as usize);

        left += 1;
        right -= 1;
    }
}

fn main() {
    let a = &mut [-1, 3, -2, 1, 2, -3, -4, -6, 4, 5, 6, -5, -7, -8, 7, 8];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");

    println!();

    let a = &mut [-1, 3, -2, 1, 2, -3, -4, -6, 4, 5, 6, -5, -7, -8, 7, 8];
    println!("{a:?}");
    let res = solve_easy(a);
    println!("{res:?}");
}
