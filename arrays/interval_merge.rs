type Range = (i32, i32);

fn merge(a: Range, b: Range) -> Option<Range> {
    let (x1, y1) = a;
    let (x2, y2) = b;

    // Check if lines are not overlapping
    if y1 < x2 || y2 < x1 {
        return None;
    }

    return Some((x1.min(x2), y1.max(y2)));
}

fn bf(a: &[Range]) -> Vec<Range> {
    let mut res = vec![];
    let n = a.len();

    let mut c = 0;

    for i in 0..n {
        if !res.is_empty() && i <= c {
            continue;
        }

        let mut temp = a[i];

        for j in i + 1..n {
            temp = if let Some(r) = merge(temp, a[j]) {
                c = j;
                r
            } else {
                c = j - 1;
                break;
            };
        }

        res.push(temp);
    }

    res
}

fn optimal(a: &[Range]) -> Vec<Range> {
    let mut res = vec![];
    let n = a.len();

    let mut i = 0;
    let mut j = 1;
    let mut temp = a[i];
    while i <= j && j < n {
        temp = if let Some(r) = merge(temp, a[j]) {
            r
        } else {
            res.push(temp);
            i = j;
            a[j]
        };

        j += 1;
    }
    res.push(temp);

    res
}

fn solve() {
    let a = &mut [
        (1, 3),
        (1, 4),
        (2, 6),
        (8, 9),
        (9, 11),
        (8, 10),
        (2, 14),
        (15, 18),
        (16, 17),
        (-1, -2),
        (100, 109),
    ];

    a.sort();
    println!("{a:?}");
    let res = bf(a);
    println!("{res:?}");
    let res = optimal(a);
    println!("{res:?}");
}

fn main() {
    solve();
}
