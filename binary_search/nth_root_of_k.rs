// multiply a number n time and get k
//

// O(n) where n is param k;
fn naive_solve(n: u32, k: u32) -> i32 {
    let mut i = 1;
    while i < k {
        // value exceed
        if i.pow(n) > k {
            break;
        }

        // get the number
        if i.pow(n) == k {
            return i as i32;
        }
        i += 1;
    }
    -1
}

// O(log n) where n is k
fn solve(n: u32, k: u32) -> i32 {
    let mut low = 1;
    let mut high = n;

    while low <= high {
        let mid = low + (high - low) / 2;

        if mid.pow(n) == k {
            return mid as i32;
        }

        if mid.pow(n) < k {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    -1
}

fn main() {
    assert_eq!(naive_solve(3, 27), 3);
    assert_eq!(naive_solve(4, 69), -1);

    assert_eq!(solve(3, 27), 3);
    assert_eq!(solve(4, 69), -1);
}
