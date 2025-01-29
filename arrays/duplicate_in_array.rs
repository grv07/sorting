// Find a duplicate item in an array of len n that conatins consicutive n-1 items.
// ex: [1,2,4,3,5,1]; n = 6, consicutive n-1 items
// sol: 1
// ex: [3,2,3,4,5,1]; n = 6, consicutive n-1 items
// sol: 3

fn bf(a: &[i32]) {
    // This solution will use O(n) space and TC is O(n)
    let mut hm = std::collections::HashMap::new();

    for i in 0..a.len() {
        if let Some(_v) = hm.get(&a[i]) {
            println!("BF: Found dupliacte for {}", a[i]);
            return;
        } else {
            hm.insert(a[i], 1);
        }
    }
}

fn better(a: &mut [i32]) {
    // This will take O(1) space but TC is O(nlogn)
    a.sort();
    for i in 0..a.len() {
        if i != 0 && a[i] == a[i - 1] {
            println!("Better: Found dupliacte for {}", a[i]);
            return;
        }
    }
}

fn optimal(a: &mut [i32]) {
    let mut i = 0;
    while a[i] != -1 {
        let temp = a[i] as usize;
        a[i] = -1;
        i = temp;
    }

    println!("Optimal: Found dupliacte for {}", i);
}

fn solve() {
    let a = &[1, 2, 4, 3, 5, 1];
    bf(a);
    println!();

    let a = &[3, 2, 4, 1, 5, 3];
    bf(a);
    println!();

    let a = &mut [1, 3, 6, 4, 1, 5, 2];
    bf(a);
    better(a);
    optimal(a);
    println!();

    let a = &mut [1, 3, 4, 2, 2];
    bf(a);
    better(a);
    optimal(a);
    println!();

    let a = &mut [3, 4, 4, 1, 5, 2];
    bf(a);
    better(a);
    optimal(a);
}

fn main() {
    solve();
}
