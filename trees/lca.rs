use tree::Node;

fn lca<T>(root: &Node<T>, t: (T, T)) -> Option<T>
where
    T: std::fmt::Display + std::cmp::PartialEq + Copy,
{
    let (a, b) = t;
    if a == root.value || b == root.value {
        return Some(root.value);
    }

    let left = if let Some(ref node) = root.left {
        lca(node, t)
    } else {
        None
    };

    let right = if let Some(ref node) = root.right {
        lca(node, t)
    } else {
        None
    };

    match (left, right) {
        (Some(_), Some(_)) => {
            println!("LCA is: {}", root.value);
            Some(root.value)
        }
        (None, Some(right)) => Some(right),
        (Some(left), None) => Some(left),
        (None, None) => None,
    }
}

pub fn solve() {
    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(4, right: binary_tree!(5, right: binary_tree!(6))),
            binary_tree!(7)
        ),
        binary_tree!(3, binary_tree!(9), binary_tree!(10))
    );

    let res = lca(&root, (4, 17));
    println!("{:?}", res);

    let res = lca(&root, (4, 7));
    println!("{:?}", res);
}
