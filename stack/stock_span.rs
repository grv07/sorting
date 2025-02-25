fn solve(a: &[i32]) -> usize {
    let mut v = vec![];
    let mut maxi = 0;

    for i in 0..a.len() {
        let mut cnt = 0;
        while let Some(last) = v.last() {
            if a[*last] <= a[i] {
                v.pop();
                if v.is_empty() {
                    cnt = i + 1;
                }
            } else {
                cnt = i - last;
                break;
            }
        }

        v.push(i);

        // print!("For {} cnt is {cnt}", a[i]);
        maxi = maxi.max(cnt);

        // println!(" maxi is {maxi}");
    }
    maxi
}

fn main() {
    let a = &[3, 2, 4, 9, 3, 1, 8, 9];
    let res = solve(a);
    println!("Res:{res}");

    let a = &[100, 80, 60, 70, 60, 75, 85];
    let res = solve(a);
    println!("Res:{res}");

    let a = &[50, 40, 30, 20, 10];
    let res = solve(a);
    println!("Res:{res}");

    let a = &[30, 30, 30, 30, 30];
    let res = solve(a);
    println!("Res:{res}");

    let a = &[30, 40, 30, 40, 30];
    let res = solve(a);
    println!("Res:{res}");

    let a = &[];
    let res = solve(a);
    println!("Res:{res}");
}
