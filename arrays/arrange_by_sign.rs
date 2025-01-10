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
}
