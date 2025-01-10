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
fn optimize() {}

fn main() {
    let a = &mut [1, 2, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2];
    better(a);
    println!("{a:?}");
}
