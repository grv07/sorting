// a: [2, 3, 4, 1, 0, 0] -> [2, 4, 3, 1, 0, 0]
fn solve(a: &mut [i32]) {
    let n = a.len();
    let mut i = n - 1;
    let mut p = -1;

    while i >= 1 {
        if a[i - 1] < a[i] {
            p = (i - 1) as i32;
            break;
        }
        i -= 1;
    }

    if p == -1 {
        // bcs given array is last permitation then reverse it to finsd first
        a.reverse();
        return;
    }

    while i < n {
        if a[i] <= a[p as usize] {
            break;
        }

        i += 1;
    }

    a.swap(p as usize, i - 1);
}

// We have a permutation of a array of string find the next permutation.
// solution:
// 1. We know next permutation will be just larger then before.
// 2. We will find the point p  where sorting in broke in the given input a.
// 3. Now to find the just next bigger combination we will replace the right arrays item that is just bigger then the
// a[p];

fn main() {
    let a = &mut [2, 3, 4, 1, 0, 0];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");
    println!();

    let a = &mut [2, 3, 3, 1, 0, 0];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");
    println!();

    let a = &mut [2, 3, 3, 1, 0, 1];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");
    println!();

    let a = &mut [2, 1, 5, 4, 3, 0, 0];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");
    println!();

    let a = &mut [4, 3, 2, 1, 0, 0];
    println!("{a:?}");
    solve(a);
    println!("{a:?}");
    println!();
}
