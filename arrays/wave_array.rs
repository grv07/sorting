pub fn solve<T>(a: &mut [T])
where
    T: Copy + std::cmp::PartialOrd,
{
    let mut i = 0;
    let mut j = 1;
    while i < j && j < a.len() {
        if a[i] > a[j] && i % 2 == 0 {
            a.swap(i, j);
        }

        if a[i] < a[j] && i % 2 != 0 {
            a.swap(i, j);
        }

        i += 1;
        j += 1;
    }
}

fn verify<T>(a: &[T]) -> bool
where
    T: Copy + std::cmp::PartialOrd,
{
    let mut i = 0;
    let mut j = 1;
    while i < j && j < a.len() {
        if a[i] == a[j] {
            return false;
        }

        if a[i] > a[j] && i % 2 == 0 {
            return false;
        }

        if a[i] < a[j] && i % 2 != 0 {
            return false;
        }

        i += 1;
        j += 1;
    }

    true
}

fn main() {
    let a = &mut [4, 3, 7, 8, 6, 2, 1];
    solve(a);
    assert!(verify(a));

    let a = &mut [4, 3, 7, 8, 6, 2, 10, 20];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [4, 7, 3, 8, 2, 1, 1, 1, 1];
    solve(a);
    println!("Res: {a:?}");
    assert_eq!(verify(a), false);

    let a = &mut [2, 8, 1, 7, 5, 9];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [1, 3, 2, 4, 3, 5];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [1, 2, 3, 4, 5, 6];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [6, 5, 4, 3, 2, 1];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [10, 90, 49, 2, 1, 5, 23];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [5, 2];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [4, 3, 7, 8, 6, 2, 1, 9];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [4, 3, 7, 8, 6, 2, 1];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [10, -5, 20, -10, 30, -15];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));

    let a = &mut [17, 42, 8, 99, 21, 5, 33];
    solve(a);
    println!("Res: {a:?}");
    assert!(verify(a));
}
