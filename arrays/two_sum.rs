fn bf(a: &[i32], t: i32) -> &str {
    for i in 0..a.len() {
        for j in (i + 1)..a.len() {
            if a[i] + a[j] == t {
                println!("{i} {j}");
                return "YES";
            }
        }
    }
    return "NO";
}

// tc: O(N) + O(1) sc: O(N)
fn better(a: &[i32], t: i32) -> &str {
    let mut hm = std::collections::HashMap::new();
    for i in 0..a.len() {
        let f = t - a[i];
        if let Some(j) = hm.get(&f) {
            println!("{i} {j}");

            return "YES";
        } else {
            hm.insert(a[i], i);
        };
    }
    return "NO";
}

// Here we will sort the array first.
// We can save some space but we can't tell the exact indexes
fn optimized(a: &[i32], t: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high = a.len() as i32;

    while low <= high {
        let mid = low + (high - low) / 2;

        if a[mid as usize] == t {
            return mid as i32;
        }

        if a[mid as usize] > t {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    return -1;
}

fn main() {
    let res = optimized(&[0, 1, 2, 3, 3, 4, 5, 6], 4);
    println!("{res}");
    let res = optimized(&[2, 4, 6, 8, 10], 6);
    println!("{res}");
    let res = optimized(&[2, 4, 6, 8, 10], 2);
    println!("{res}");
    let res = optimized(&[2, 4, 6, 8, 10], 7);
    println!("{res}");
    let res = optimized(&[7], 7);
    println!("{res}");
    let res = optimized(&[7], 2);
    println!("{res}");
    let res = optimized(&[1, 9], 2);
    println!("{res}");

    println!("---");
    let res = bf(&[1, 2, 3, 4], 4);
    println!("{res}");
    let res = better(&[1, 2, 3, 4], 4);
    println!("{res}");
    println!("");

    let res = bf(&[4, 1, 2, 3, 1], 5);
    println!("{res}");
    let res = better(&[4, 1, 2, 3, 1], 5);
    println!("{res}");

    println!("");

    let res = bf(&[2, 6, 5, 8, 11], 14);
    println!("{res}");
    let res = better(&[2, 6, 5, 8, 11], 14);
    println!("{res}");
}
