// input = [1, 2, 3]
// 1 2 3
// 1 3 2
//
// 2 1 3
// 2 3 1
//
// 3 1 2
// 3 2 1
//
// 3! = 6
// k = 5
// 6/3 = 2
// 5 / 2 = 2
// input[2] = 3
// [3, ]
// k = 2;
// 2! = 2
// 2/2 = 1
// 1/2 = 1
//

fn fact(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    let res = n * fact(n - 1);
    res
}

fn k_perm(input: &mut Vec<usize>, res: &mut Vec<usize>, k: usize) {
    let n = input.len() - 1;
    let t = k / fact(n);

    res.push(input.remove(t));

    if n == 0 {
        println!("{res:?}");
        return;
    }

    k_perm(input, res, k % fact(n));
}

fn main() {
    let _res = fact(4);

    let mut res = vec![];
    let mut input = vec![1, 2, 3, 4];
    k_perm(&mut input, &mut res, 16);
}
