fn bf(a: &[i32], t: i32) {
    println!("{a:?}");

    let n = a.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    let sum = a[i] + a[j] + a[l] + a[k];
                    if sum == t {
                        println!("{} {} {} {}", a[i], a[j], a[k], a[l]);
                    }
                }
            }
        }
    }
}

fn optimal(a: &mut [i32]) {
    a.sort();
    println!("{a:?}");

    let n = a.len();
    for i in 0..n {
        let j = i + 1;
        let mut k = j + 1;
        let mut l = n - 1;

        while k < l {
            let t = a[i] + a[j] + a[k] + a[l];

            if t > 0 {
                l -= 1;
            } else if t < 0 {
                k += 1;
            } else {
                println!("{} {} {} {}", a[i], a[j], a[k], a[l]);
                while a[k] == a[k - 1] && k < l {
                    k += 1;
                }
                while a[l] == a[l + 1] && l > j {
                    l -= 1;
                }

                k += 1;
                l -= 1;
            }
        }
    }
}

fn solve() {
    let a = &mut [1, 2, 3, 4, 4, 5, -1, -2, 0];
    println!("BF: ");
    bf(a, 0);

    println!("\nOPTIMAL: ");
    optimal(a);
}

fn main() {
    solve();
}
