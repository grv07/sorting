// How we set the indexes in array
//     2*i
// 2*i+1  2*i+2

fn max_heapify(a: &mut Vec<i32>) {
    let mut i = 0;
    let n = a.len();

    while i < n / 2 {
        let mut maxi = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;

        if l < n && a[i] < a[l] {
            maxi = l;
        }

        if r < n && a[maxi] < a[r] {
            maxi = r;
        }

        if maxi == i {
            break;
        }

        a.swap(i, maxi);
        i = maxi;
    }
}

fn main() {
    let mut a = vec![1, 2, 3, 5, 6];
    max_heapify(&mut a);
    assert_eq!(a, vec![3, 2, 1, 5, 6]);

    let mut a = vec![1, 3, 2, 5, 6];
    max_heapify(&mut a);
    assert_eq!(a, vec![3, 6, 2, 5, 1]);

    let mut a = vec![10, 5, 6, 2, 3];
    max_heapify(&mut a);
    assert_eq!(a, vec![10, 5, 6, 2, 3]);

    let mut a = vec![3, 5, 2, 1, 0];
    max_heapify(&mut a);
    assert_eq!(a, vec![5, 3, 2, 1, 0]);

    let mut a = vec![3, 1, 6, 0, 2];
    max_heapify(&mut a);
    assert_eq!(a, vec![6, 1, 3, 0, 2]);

    let mut a = vec![1, 3, 2, 5, 6];
    max_heapify(&mut a);
    assert_eq!(a, vec![3, 6, 2, 5, 1]);

    let mut a = vec![4, 4, 4, 4, 4];
    max_heapify(&mut a);
    assert_eq!(a, vec![4, 4, 4, 4, 4]);

    let mut a = vec![42];
    max_heapify(&mut a);
    assert_eq!(a, vec![42]);
}
