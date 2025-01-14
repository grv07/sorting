// Hare's algorithum
fn pivot<T: std::cmp::PartialOrd + std::fmt::Debug>(a: &mut [T], pivot: usize) {
    let mut i = 0;
    let mut j = a.len() - 1;
    // 3, 1, 4, 6, 2
    // i = 2
    // j = 4
    // swap(2, 4)
    // 3, 1, 2, 6, 4
    // i = 2, j = 1
    while i < j {
        while a[i] <= a[pivot] && i < a.len() - 1 {
            i += 1;
        }

        while a[j] > a[pivot] && j > 0 {
            j -= 1;
        }

        if j <= i {
            break;
        }

        a.swap(i, j);
    }

    a.swap(pivot, j);
    // println!("{a:?}");
}

fn quick_sort(a: &mut [i32]) {
    for i in 0..a.len() {
        pivot(a, i);
    }
    println!("{a:?}");
}

fn sequence_on_array<T>(a: &[T])
where
    T: Copy
        + std::fmt::Debug
        + PartialOrd
        + std::ops::Add<i32, Output = T>
        + std::ops::AddAssign<i32>,
{
    let mut f_res = vec![];
    for i in a {
        let mut res = vec![i.clone()];
        let mut q = *i + 1;
        let mut p = 0;

        while p < a.len() {
            if a[p] == q {
                res.push(q);
                q += 1;
                p = 0;
            } else {
                p += 1;
            }
        }

        if f_res.len() < res.len() {
            f_res = res;
        }
    }
    println!("{f_res:?}");
}

fn better(a: &mut [i32]) {
    if a.len() == 0 {
        return;
    }

    quick_sort(a);

    let mut last = a[0];
    let mut max_cnt = 1;
    let mut cnt = 1;
    for i in 0..a.len() {
        if a[i] == last {
            continue;
        }

        if a[i] == last + 1 {
            last = a[i];
            cnt += 1;
        } else {
            last = a[i];
            cnt = 1;
        }

        if cnt > max_cnt {
            max_cnt = cnt;
        }
    }

    println!("{max_cnt}");
}

fn optimize(a: &[i32]) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for i in a {
        map.entry(i).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut res = vec![];

    for i in a {
        let mut temp = vec![];

        if let Some(_) = map.get(&(i - 1)) {
            continue;
        }

        let mut q = i.clone();
        loop {
            if let Some(_) = map.get(&q) {
                temp.push(q);
                q += 1;

                continue;
            }
            break;
        }

        if temp.len() > res.len() {
            res = temp;
        }
    }

    println!("{res:?}");
    return res;
}

fn main() {
    sequence_on_array(&[1, 2, 3]);
    better(&mut [1, 2, 3]);
    optimize(&[1, 2, 3]);
    println!();

    sequence_on_array(&[103, 2, 3, 100, 102, 1001, 101, 99]);
    better(&mut [103, 2, 3, 100, 102, 1001, 101, 99]);
    optimize(&[103, 2, 3, 100, 102, 1001, 101, 99]);
    println!();

    sequence_on_array(&[1, 5, 3, 4]);
    better(&mut [1, 5, 3, 4]);
    optimize(&[1, 5, 3, 4]);
    println!();

    sequence_on_array::<i32>(&[]);
    better(&mut []);
    optimize(&[]);
    println!();

    sequence_on_array(&[1]);
    better(&mut [1]);
    optimize(&[1]);
    println!();

    sequence_on_array(&[1, 3, 5]);
    better(&mut [1, 3, 5]);
    optimize(&[1, 3, 5]);
    println!();

    sequence_on_array(&[1, 4, 6, 2]);
    better(&mut [1, 4, 6, 2]);
    optimize(&[1, 4, 6, 2]);
    println!();

    quick_sort(&mut [3, 1, 4, 6, 2]);
    quick_sort(&mut [3, 1, 4, 6, 2, 1, 1, 1, 0, 12, 23, 22, 3, 4, 9]);
    quick_sort(&mut [3, 1, 4, 6, 2, 1, 1, 1, 0, 12, 23, 22, 3, 4, 9, -34]);
    quick_sort(&mut [3, 4, 6, 12, 23, 22, 3, 4, 9]);
}
