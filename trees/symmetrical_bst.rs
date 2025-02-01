#[allow(unused_imports)]
use tree::{print_stack, Node};

fn is_symmetrical<T: std::cmp::PartialOrd>(root: &Node<T>) -> bool
where
    T: std::fmt::Debug,
{
    let mut s: Vec<&Node<T>> = vec![];

    if let (Some(n1), Some(n2)) = (&root.left, &root.right) {
        s.push(&*n1);
        s.push(&*n2);
    }

    while let (Some(ref left), Some(ref right)) = (s.pop(), s.pop()) {
        if left.value == right.value {
            match (&left.right, &right.left) {
                (Some(n1), Some(n2)) => {
                    s.insert(0, &*n1);
                    s.insert(0, &*n2);
                }
                (None, None) => {}
                (_, _) => return false,
            }

            match (&left.left, &right.right) {
                (Some(ref n1), Some(ref n2)) => {
                    s.insert(0, &*n1);
                    s.insert(0, &*n2);
                }
                (None, None) => {}
                (_, _) => return false,
            }
        } else {
            return false;
        }
        // print_stack(s.clone());
    }

    true
}

pub fn solve() {
    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(7),
            binary_tree!(4, left: binary_tree!(5, left: binary_tree!(6)))
        ),
        binary_tree!(
            2,
            binary_tree!(4, right: binary_tree!(5, right: binary_tree!(6))),
            binary_tree!(7)
        )
    );

    let res = is_symmetrical(&root);
    println!("Is tree symmetrical: {res:?}");

    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(7),
            binary_tree!(4, left: binary_tree!(5, left: binary_tree!(6)))
        ),
        binary_tree!(
            2,
            binary_tree!(4, right: binary_tree!(5, right: binary_tree!(6))),
            binary_tree!(7, left: binary_tree!(23))
        )
    );

    let res = is_symmetrical(&root);
    println!("Is tree symmetrical: {res:?}");
}
