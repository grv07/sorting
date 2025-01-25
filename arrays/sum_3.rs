fn bf(a: &[i32], t: i32) {
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            for k in j + 1..a.len() {
                if a[i] + a[j] + a[k] == t {
                    println!("{} {} {}", a[i], a[j], a[k]);
                }
            }
        }
    }
}

fn better(a: &[i32], t: i32) {
    let mut hm = std::collections::HashMap::new();

    for i in 0..a.len() {
        hm.clear();

        for j in i + 1..a.len() {
            let t = -(a[j] + a[i]);
            if let Some(_) = hm.get(&t) {
                println!("{} {} {}", a[i], a[j], t);
            }

            hm.insert(&a[j], 1);
        }
    }
}

fn optimal(a: &[i32], t: i32) {
    for i in 0..a.len() {
        if i > 0 && a[i] == a[i - 1] {
            continue;
        }

        let mut k = a.len() - 1;
        let mut j = i + 1;

        while j < k {
            let t = a[i] + a[j] + a[k];

            if t > 0 {
                k -= 1;
            } else if t < 0 {
                j += 1;
            } else {
                println!("{} {} {}", a[i], a[j], a[k]);
                j += 1;
                k -= 1;

                while j < k && a[j] == a[j - 1] {
                    j += 1;
                }
                while j < k && a[k] == a[k + 1] {
                    k -= 1;
                }
            }
        }
    }
}

fn main() {
    let a = &[-1, 0, 1, 2, -1, -4];

    bf(a, 0);

    println!("\n");
    better(a, 0);

    println!("\n");

    let a = &mut [-4, -2, -1, -1, -1, -1, 0, 1, 2, 2, 2, 3];
    a.sort();
    optimal(a, 0);
}
