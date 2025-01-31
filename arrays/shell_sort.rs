fn sort<T>(a: &mut [T])
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    let mut gap = (a.len() + 1) / 2;

    while gap > 0 {
        let mut i = 0;
        while (i + gap) < a.len() {
            if a[i] > a[i + gap] {
                a.swap(i, i + gap);
            }
            i += 1;
        }

        gap = gap / 2;
    }
}

fn main() {
    let a = &mut [-1, 2, 3, 4, -15, 26, 25, -1];
    sort(a);
    println!("{a:?}");

    let a = &mut [1, 2, 3, 4, 5];
    sort(a);
    println!("{a:?}");

    let a = &mut [5, 4, 3, 2, 1];
    sort(a);
    println!("{a:?}");

    let a = &mut [12, 34, 54, 2, 3];
    sort(a);
    println!("{a:?}");

    let a = &mut [1, 5, 3, 3, 2];
    sort(a);
    println!("{a:?}");

    let a = &mut [7, 7, 7, 7, 7];
    sort(a);
    println!("{a:?}");
}
