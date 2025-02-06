// Find the minimim prod of two items from array A and array B
// ex:
// A = [1, 1, 3]
// B = [6, 5, 4]
// ans: 1*6+1*5+3*4

fn solve<T>(a: &mut [T], b: &mut [T]) -> T
where
    T: std::cmp::Ord
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Default
        + Copy
        + std::fmt::Debug,
{
    a.sort();
    b.sort();

    let n = a.len();
    let mut sum = T::default();

    for i in 0..n {
        println!("{:?} {:?}", a[i], b[n - (i + 1)]);
        sum = sum + (a[i] * b[n - 1 - i]);
    }

    sum
}

fn main() {
    let res = solve(&mut [1, 1, 3], &mut [6, 5, 4]);
    println!("Res: {res}");

    let mut arr1 = vec![1, 3, 5, 2];
    let mut arr2 = vec![4, 2, 3, 1];
    let res = solve(&mut arr1, &mut arr2);
    println!("Res: {res}");

    let mut arr1 = vec![-1, -3, -5, -2];
    let mut arr2 = vec![4, 2, 3, 1];
    let res = solve(&mut arr1, &mut arr2);
    println!("Res: {res}");
}
