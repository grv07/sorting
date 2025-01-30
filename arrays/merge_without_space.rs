fn merge<T>(a: &mut [T], b: &mut [T])
where
    T: std::fmt::Debug + std::cmp::PartialOrd + Copy + std::cmp::Ord,
{
    let mut i = a.len();
    let mut j = 0;
    while i > 0 && j < b.len() {
        i -= 1;

        if a[i] > b[j] {
            let temp = a[i];
            a[i] = b[j];
            b[j] = temp;
        }

        j += 1;
    }

    // Sort both sides of the array
    a.sort();
    b.sort();
}

fn main() {
    let a = &mut [1, 2, 4, 7];
    let b = &mut [3, 4, 5, 5, 6];
    merge(a, b);
    println!("{a:?} {b:?}");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [3, 4, 5, 5, 6];
    merge(a, b);
    println!("{a:?} {b:?}");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 1, 1, 1];
    merge(a, b);
    println!("{a:?} {b:?}");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 2, 3, 4];
    merge(a, b);
    println!("{a:?} {b:?}");

    let a = &mut [1, 2, 7, 7, 7, 7];
    let b = &mut [2, 3, 4];
    merge(a, b);
    println!("{a:?} {b:?}");
}
