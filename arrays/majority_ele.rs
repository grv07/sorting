// Give two or n/3 freq elements from un sorted array

fn best(a: &[i32]) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();

    let n = a.len() / 3;
    let mut res = vec![];

    for i in 0..a.len() {
        if res.len() == 2 {
            break;
        }

        hm.entry(&a[i])
            .and_modify(|v| {
                *v += 1;
                if *v == n + 1 {
                    res.push(a[i]);
                }
            })
            .or_insert(1);
    }

    res
}

fn optimize<
    T: Copy + std::cmp::PartialOrd + std::ops::Add<i32, Output = i32> + Into<i32> + From<i32>,
>(
    a: &[T],
) -> Vec<T> {
    let mut e1 = 0;
    let mut e2 = 0;
    let mut c1 = 0;
    let mut c2 = 0;

    for i in 0..a.len() {
        if c1 == 0 && e2 != a[i].into() {
            e1 = a[i].into();
            c1 = 1;
        } else if c2 == 0 && e1 != a[i].into() {
            e2 = a[i].into();
            c2 = 1;
        } else if e1 == a[i].into() {
            c1 += 1;
        } else if e2 == a[i].into() {
            c2 += 1;
        } else {
            c1 -= 1;
            c2 -= 1;
        }
    }

    c1 = 0;
    c2 = 0;
    let mut res = vec![];
    let limit = (a.len() / 3) as i32 + 1;

    for i in a {
        if *i == e1.into() {
            c1 += 1;
        }
        if *i == e2.into() {
            c2 += 1;
        }

        if c1 == limit {
            res.push(e1.into());
        }
        if c2 == limit {
            res.push(e2.into());
        }
    }

    res
}

fn majority_ele(a: &[i32]) {
    let o_res = optimize(a);
    println!("{:?}", o_res);

    let b_res = best(a);
    println!("{:?}", b_res);
}

fn main() {
    let a = &[1, 2, 1, 3, 4, 1, 1, 1, 1, 2, 2, 2];
    majority_ele(a);
}
