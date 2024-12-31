use std::collections::HashMap;

fn remove(x: char, b: &mut HashMap<&char, i32>) -> bool {
    if let Some(v) = b.get_mut(&x) {
        *v -= 1;
    }
    b.values().all(|v| *v >= 0)
}

fn add(x: char, b: &mut HashMap<&char, i32>) -> bool {
    if let Some(v) = b.get_mut(&x) {
        *v += 1;
    }
    b.values().all(|v| *v >= 0)
}

fn solve(input: &[char], v: &[char]) {
    let mut b = HashMap::new();
    for c in v {
        b.entry(c).and_modify(|x: &mut i32| *x -= 1).or_insert(-1);
    }

    let mut i = 0;
    let mut j = 0;
    let mut state = false;

    while i <= j {
        if state {
            println!("State is true {i} {}", j - 1);
        }

        if j >= input.len() {
            return;
        }

        if state {
            state = remove(input[i], &mut b);
            i += 1;
        }

        if !state {
            state = add(input[j], &mut b);
            j += 1;
        }
    }
}

fn main() {
    solve(&['a', 'b', 'b', 'a', 'b'], &['b', 'b']);
    println!("");
    solve(&['a', 'b', 'b', 'a', 'b', 'b', 'b'], &['b', 'b']);
}
