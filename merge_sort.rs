fn main() {
    let res = merge_sort(vec![1, 2, 4, 3].as_ref());
    let _res = merge_sort(vec![1, 3, 2, 4, 6, 5, 89, 45].as_ref());
    let _res = merge_sort(vec![1].as_ref());
    let _res = merge_sort(vec![1, 0].as_ref());
    println!("{:?}", res);
}

fn merge_sort(input: &[i32]) -> Vec<i32> {
    let n = input.len();

    if n <= 1 {
        return input.to_vec();
    }

    let (left, right) = input.split_at(n / 2);

    let mut left_i = merge_sort(left).into_iter();
    let mut right_i = merge_sort(right).into_iter();

    let mut res = vec![];
    let mut left = left_i.next();
    let mut right = right_i.next();
    for _ in 0..(left_i.len() + right_i.len() + 2) {
        match (left, right) {
            (Some(l), Some(r)) => {
                if l < r {
                    res.push(l);
                    left = left_i.next();
                } else {
                    res.push(r);
                    right = right_i.next();
                }
            }
            (None, Some(r)) => {
                res.push(r);
                right = right_i.next();
            }
            (Some(l), None) => {
                res.push(l);
                left = left_i.next();
            }

            (None, None) => break,
        }
    }

    res
}
