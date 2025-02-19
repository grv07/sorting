#[derive(std::cmp::PartialEq)]
enum Move {
    Left,
    Right,
}

fn get_move(i: i32) -> Move {
    if i < 0 {
        Move::Left
    } else {
        Move::Right
    }
}

fn solve(a: &[i32]) -> Vec<i32> {
    let mut s = vec![];

    for i in 0..a.len() {
        let m = get_move(a[i]);
        if s.is_empty() {
            s.push(a[i]);
            continue;
        }

        if let Some(last) = s.last() {
            if m == get_move(*last) {
                s.push(a[i]);
                continue;
            }
        }

        while let Some(l) = s.last() {
            if m != get_move(*l) {
                let current = a[i].abs();
                let last = l.abs();
                // println!(" >> {} {} {s:?}", a[i], l);

                if last == current {
                    s.pop();
                    break;
                }

                if !s.is_empty() && last < current {
                    s.pop();
                    if s.is_empty() {
                        s.push(a[i]);
                        break;
                    }
                    continue;
                }
            }

            break;
        }
    }

    s
}

fn main() {
    let res = solve(&[1, 7, 2, 3, -7, -9, 10]);
    println!("{res:?}");

    let res = solve(&[4, 7, 1, 1, 2, -3, -7, 17, 15, -16]);
    println!("{res:?}");
}
