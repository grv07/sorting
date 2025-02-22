fn nse(a: &[i32]) -> Vec<i32> {
    let n = a.len();

    let mut s = vec![];
    let mut res = vec![n as i32; n];

    for i in (0..a.len()).rev() {
        while let Some(last) = s.last() {
            if a[*last as usize] < a[i] {
                res[i] = *last;
                break;
            }
            s.pop();
        }

        s.push(i as i32);
    }

    res
}

fn pse(a: &[i32]) -> Vec<i32> {
    let n = a.len();

    let mut s = vec![];
    let mut res = vec![-1; n];

    for i in 0..a.len() {
        while let Some(last) = s.last() {
            if a[*last as usize] < a[i] {
                res[i] = *last;
                break;
            }
            s.pop();
        }

        s.push(i as i32);
    }

    res
}

fn bf(a: &[i32]) -> i32 {
    let n = a.len();
    let mut maxi = 0;

    for i in 0..a.len() {
        let mut l = i as i32 - 1;
        while l >= 0 {
            if a[i] > a[l as usize] {
                break;
            }
            l -= 1;
        }
        if l < 0 {
            l = -1;
        }

        let mut r = i + 1;
        while r < n {
            if a[i] > a[r] {
                break;
            }
            r += 1;
        }
        if r >= n {
            r = n;
        }

        maxi = maxi.max((r as i32 - l - 1) * a[i]);
    }
    maxi
}

fn two_stack(a: &[i32]) -> i32 {
    let nse = nse(a);
    // println!("NSE Res: {nse:?}");
    let pse = pse(a);
    // println!("PSE Res: {pse:?}");

    let mut maxi = 0;

    for i in 0..nse.len() {
        let p = nse[i] - pse[i] - 1;
        maxi = maxi.max(p * a[i]);
    }

    maxi
}

fn single_stack(a: &[i32]) -> i32 {
    let n = a.len();
    let mut v = vec![];
    let mut nse = vec![n as i32; n];
    let mut pse = vec![-1; n];
    let mut maxi = 0;

    for i in 0..a.len() {
        while let Some(last) = v.last() {
            if a[*last as usize] > a[i] {
                nse[*last as usize] = i as i32;
                maxi = maxi.max((i as i32 - pse[*last as usize] - 1) * a[*last as usize]);
                v.pop();
            } else {
                pse[i] = *last;
                break;
            }
        }

        v.push(i as i32);
    }

    println!("V: {v:?}");
    while let Some(tp) = v.pop() {
        let pse = if let Some(last) = v.last() { *last } else { -1 };

        maxi = maxi.max((n as i32 - pse - 1) * a[tp as usize]);
    }

    println!("PSE: {pse:?} \nNSE: {nse:?} \n{maxi}");
    maxi
}

fn solve(a: &[i32]) {
    println!("INPUT:   {a:?}");
    let res = bf(a);
    println!("BF Res: {res}");

    let res = two_stack(a);
    println!("Two Stack Res: {res}");

    let res = single_stack(a);
    println!("Single Stack Res: {res}");
}

fn main() {
    solve(&[3, 1, 2, 4, 5, 6, 3, 6, 6]);
    let res = single_stack(&[1, 0, 1, 0, 1]);
    println!("Single Stack Res: {res}");
}
