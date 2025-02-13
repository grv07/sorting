fn solve(a: &[i32]) -> i32 {
    let mut cr = 0;
    let mut mr = 0;
    let mut jump = 0;

    for i in 0..a.len() {
        // print!("i: {i} cr: {cr} mr: {mr} j: {jump} > ");
        mr = mr.max(i + a[i] as usize);

        if mr >= a.len() - 1 {
            return jump + 1;
        }

        if i == cr {
            if i == mr {
                return -1;
            } else {
                jump += 1;
                cr = mr;
            }
        }
        // println!("i: {i} cr: {cr} mr: {mr} j: {jump}");
    }

    -1
}

fn main() {
    let a = &[1, 2, 1, 3, 0, 3, 3, 0, 0, 1];
    let res = solve(a);

    println!("Res: {res}");
}
