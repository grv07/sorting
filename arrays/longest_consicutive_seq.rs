fn sequence_on_array(a: &[i32]) {
    let mut f_res = vec![];
    for i in a {
        let mut res = vec![i.clone()];
        let mut q = i + 1;
        let mut p = 0;

        while p < a.len() {
            if a[p] == q {
                res.push(q);
                q += 1;
                p = 0;
            } else {
                p += 1;
            }
        }

        if f_res.len() < res.len() {
            f_res = res;
        }
    }
    println!("{f_res:?}");
}

fn main() {
    sequence_on_array(&[1, 2, 3]);

    sequence_on_array(&[103, 2, 3, 100, 102, 1001, 101, 99]);

    sequence_on_array(&[1, 5, 3]);
}
