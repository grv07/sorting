fn merge_by_sort<T>(a: &mut [T], b: &mut [T])
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

fn set_v<T: Copy>(idx: usize, v: T, a: &mut [T], b: &mut [T]) {
    if idx < a.len() {
        a[idx] = v;
    } else {
        let idx = idx - a.len();
        b[idx] = v;
    }
}

fn get_v<T: Copy>(idx: usize, a: &[T], b: &[T]) -> T {
    if idx < a.len() {
        return a[idx];
    } else {
        let idx = idx - a.len();
        return b[idx];
    }
}

fn merge_by_gap<T>(a: &mut [T], b: &mut [T])
where
    T: Copy + std::cmp::PartialOrd + std::fmt::Debug,
{
    // println!(">> {a:?} {b:?}");
    let m = a.len() + b.len() - 1;
    let mut gap = (a.len() + b.len() + 1) / 2;

    while gap > 0 {
        // println!("gap: {gap}");
        for i in 0..m {
            if (i + gap) > m {
                break;
            }

            let ga = get_v(i, a, b);
            let gb = get_v(i + gap, a, b);
            // println!(">> {ga:?} {gb:?}");
            if ga > gb {
                let tmp = gb;
                set_v(i + gap, ga, a, b);
                set_v(i, tmp, a, b);
            }
        }
        // println!(">> {a:?} {b:?}");
        gap = (gap) / 2;
    }
}

fn _solve(a: &mut [i32], b: &mut [i32]) {
    println!("Input: \n");
    println!("{a:?}\n{b:?}");

    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");

    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?} \n");
}

fn main() {
    let a = &mut [1, 2, 4, 7];
    let b = &mut [3, 4, 5, 5, 6];
    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");
    let a = &mut [1, 2, 4, 7];
    let b = &mut [3, 4, 5, 5, 6];
    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?} \n");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [3, 4, 5, 5, 6];
    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");
    let a = &mut [1, 2, 3, 3];
    let b = &mut [3, 4, 5, 5, 6];
    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?} \n");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 1, 1, 1];
    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");
    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 1, 1, 1];
    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?} \n");

    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 2, 3, 4];
    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");
    let a = &mut [1, 2, 3, 3];
    let b = &mut [1, 2, 3, 4];
    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?} \n");

    let a = &mut [1, 2, 7, 7, 7, 7];
    let b = &mut [2, 3, 4];
    merge_by_sort(a, b);
    println!("By sort {a:?} {b:?}");
    let a = &mut [1, 2, 7, 7, 7, 7];
    let b = &mut [2, 3, 4];
    merge_by_gap(a, b);
    println!("By gap  {a:?} {b:?}");
}
