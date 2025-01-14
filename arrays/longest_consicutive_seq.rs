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
    optimize(&[1, 2, 3]);

    sequence_on_array(&[103, 2, 3, 100, 102, 1001, 101, 99]);
    optimize(&[103, 2, 3, 100, 102, 1001, 101, 99]);

    sequence_on_array(&[1, 5, 3, 4]);
    optimize(&[1, 5, 3, 4]);

    sequence_on_array::<i32>(&[]);
    optimize(&[]);

    sequence_on_array(&[1]);
    optimize(&[1]);

    sequence_on_array(&[1, 3, 5]);
    optimize(&[1, 3, 5]);

    sequence_on_array(&[1, 4, 6, 2]);
    optimize(&[1, 4, 6, 2]);
}
