fn solve(a: &[i32], k: usize) -> i32 {
    let mut q = std::collections::VecDeque::new();
    let mut sum = 0;

    for i in 0..a.len() {
        if let Some(head) = q.front() {
            if i >= (*head + k) {
                // println!("Maximum for pos {i} {:?}", a[*head]);
                q.pop_front();
            }
        }

        while let Some(last) = q.back() {
            if a[*last] < a[i] {
                q.pop_back();
                continue;
            }

            break;
        }

        q.push_back(i);

        if let Some(head) = q.front() {
            if i >= k - 1 {
                sum += a[*head];
                // println!("Maximum for pos {i} {:?}", a[*head]);
            }
        }
    }

    sum
}

fn main() {
    let a = &[12, 1, 2, -3, 10, 5, 7];
    let res = solve(a, 3);
    println!("Res: {res:?}");

    let a = &[12, 1, 2, -3, 10, 5, 7];
    let res = solve(a, 2);
    println!("Res: {res:?}");
}
