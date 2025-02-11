fn merge_sort_rs(a: &mut [i32]) -> usize {
    let mut cnt = 0;

    let n = a.len();
    if n <= 1 {
        return cnt;
    }

    let mid = n / 2;

    cnt += merge_sort_rs(&mut a[..mid]);
    cnt += merge_sort_rs(&mut a[mid..]);

    cnt += merge_rs(a, mid);

    cnt
}

fn merge_rs(a: &mut [i32], mid: usize) -> usize {
    let (mut i, mut j) = (0, mid);
    let mut res = vec![];
    let mut cnt = 0;

    while i < mid && j < a.len() {
        if a[i] <= a[j] {
            res.push(a[i]);
            i += 1;
        } else {
            // [1,2,3,2]
            // i m j
            cnt += mid - i;
            res.push(a[j]);
            j += 1;
        }
    }

    while i < mid {
        res.push(a[i]);
        i += 1;
    }

    while j < a.len() {
        res.push(a[j]);
        j += 1;
    }

    for i in 0..res.len() {
        a[i] = res[i];
    }

    cnt
}

fn main() {
    let a = &mut [5, 3, 2];
    println!("Inp: {a:?}");
    let cnt = merge_sort_rs(a);
    println!("Res: {a:?} {cnt}");

    let a = &mut [5, 3, 2, 1, 4];
    println!("Inp: {a:?}");
    let cnt = merge_sort_rs(a);
    println!("Res: {a:?} {cnt}");
}
