// bf: approch will be to do sort n log(n)

// Better is O(N) + O(N)
fn better(a: &mut [i32]) {
    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;

    for i in 0..a.len() {
        if a[i] == 1 {
            ones += 1;
        } else if a[i] == 2 {
            twos += 1;
        } else {
            // here
            threes += 1;
        }
    }

    for i in 0..a.len() {
        if ones != 0 {
            a[i] = 1;
            ones -= 1;
        } else if twos != 0 {
            a[i] = 2;
            twos -= 1;
        } else {
            a[i] = 3;
            threes -= 1;
        }
    }
}

// Dutch national flag
fn optimize(a: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = a.len() - 1;

    while a[k] == 3 {
        k -= 1;
    }

    while a[i] == 1 {
        i += 1;
        j += 1;
    }

    while i <= j && j < k {
        if a[j] == 3 {
            a.swap(j, k);
            k -= 1;
        }

        if a[j] == 1 {
            a.swap(i, j);
            i += 1;
        }

        j += 1;
    }
}

fn main() {
    let a = &mut [1, 2, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2];
    better(a);
    println!("{a:?}");

    let a = &mut [
        2, 2, 1, 2, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2, 3,
    ];
    optimize(a);
    println!("{a:?}");
}
