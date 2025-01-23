fn majority_ele<
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

    vec![e1.into(), e2.into()]
}

fn main() {
    let a = &[1, 2, 1, 3, 4, 1, 1, 1, 1, 2, 2, 2];
    let res = majority_ele(a);
    println!("{:?}", res);
}
