fn transpose(a: &mut [&mut [i32]]) {
    let n = a.len();

    let mut l = 0;
    for r in l..n {
        for c in l..n {
            if c == r {
                continue;
            }
            let temp = a[r][c];
            let column = a[c][r];

            a[r][c] = column;
            a[c][r] = temp;
        }
        l += 1;
    }
}

fn rotate(a: &mut [&mut [i32]]) {
    transpose(a);
    for i in 0..a.len() {
        a[i].reverse();
    }
}

fn dump(a: &mut [&mut [i32]]) {
    for i in a {
        for j in &**i {
            print!("{j} ");
        }
        println!();
    }
    println!("----------");
}

fn main() {
    let input: &mut [&mut [i32]] = &mut [
        &mut [1, 2, 3, 4],
        &mut [5, 6, 7, 8],
        &mut [9, 10, 11, 12],
        &mut [13, 14, 15, 16],
    ];
    dump(input);
    rotate(input);
    // transpose(input);
    dump(input);

    let a = &mut [1, 2, 3, 4, 5, 6, 1, 2, 3, 1];
    quick_sort(a);
    println!("{a:?}");

    let a = &mut [1, 2, 3, 3, 14, 15, 16, 1, 2, 3, 1, 13];
    quick_sort(a);
    println!("{a:?}");

    let a = &mut [];
    quick_sort(a);
    println!("{a:?}");
}

fn quick_sort(a: &mut [i32]) {
    let n = a.len();

    if n <= 1 {
        return;
    }

    let piv = partitiion(a);

    quick_sort(&mut a[0..piv]);
    quick_sort(&mut a[piv + 1..n]);
}

fn partitiion(a: &mut [i32]) -> usize {
    if a.is_empty() {
        return 0;
    }

    let n = a.len();
    let mut i = 0;
    let pivot = a[n - 1];

    for it in 0..n {
        if a[it] < pivot {
            a.swap(it, i);
            i += 1;
        }
    }
    a.swap(i, n - 1);
    i
}
