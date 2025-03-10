// The Idea is to do InOrder and count the Kth and return.
use tree::Node;

fn in_order<T: Copy + std::fmt::Debug>(root: &Node<T>, k: i32) -> Option<T> {
    let mut v = vec![Some(root)];
    let mut cnt = 0;

    loop {
        if let Some(Some(last)) = v.last() {
            v.push(last.left.as_deref());
        } else {
            if let (Some(None), Some(Some(last))) = (v.pop(), v.pop()) {
                v.push(last.right.as_deref());
                cnt += 1;
                if cnt == k {
                    return Some(last.value);
                }
            }
        }

        if v.is_empty() {
            break;
        }
    }

    None
}

fn in_order_rev<T: Copy + std::fmt::Debug>(root: &Node<T>, k: i32) -> Option<T> {
    let mut v = vec![Some(root)];
    let mut cnt = 0;

    loop {
        if let Some(Some(last)) = v.last() {
            v.push(last.right.as_deref());
        } else {
            if let (Some(None), Some(Some(last))) = (v.pop(), v.pop()) {
                v.push(last.left.as_deref());
                cnt += 1;
                if cnt == k {
                    return Some(last.value);
                }
            }
        }

        if v.is_empty() {
            break;
        }
    }

    None
}

pub fn solve() {
    let root = binary_tree!(
        5, // Root
        binary_tree!(
            // Left subtree
            3,
            binary_tree!(2, left: binary_tree!(1)), // Left-only
            binary_tree!(4)                         // Leaf
        ),
        binary_tree!(
            // Right subtree
            8,
            binary_tree!(7, left: binary_tree!(6)), // Left-only
            binary_tree!(9, right: binary_tree!(10))  // Right-only
        )
    );

    let res = in_order(&root, 10);

    println!("Kth value in BST: {res:?}");

    let res = in_order_rev(&root, 8);

    println!("Kth value in BST: {res:?}");
}
