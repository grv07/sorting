fn sort<T>(a: &mut [T])
where
    T: std::cmp::PartialOrd,
{
    let n = a.len();
    for i in 1..n {
        let mut j = 0;
        while j < i {
            if a[j] > a[i] {
                a.swap(i, j);
            }
            j += 1;
        }
    }
}

fn main() {
    let a = &mut [1, 2, 3, 4, 5, 2, 3, 1];
    sort(a);
    println!("{a:?}");

    let a = &mut [1, -2, 13, 40, 5, 2, 3, -1];
    sort(a);
    println!("{a:?}");
}
