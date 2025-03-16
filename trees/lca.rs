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

fn lca_iter<T>(root: &Node<T>, t: (T, T))
where
    T: std::fmt::Display + std::cmp::PartialEq + Copy + std::fmt::Debug,
{
    let mut v = vec![];
    let mut v1 = vec![root];

    while let Some(node) = v1.pop() {
        v.push(node);

        if let Some(node) = &node.right {
            v1.push(node);
        }
        if let Some(node) = &node.left {
            v1.push(node);
        }
    }

    let mut check = (false, false);
    for i in v.into_iter().rev() {
        if check == (true, true) {
            println!("LCA for {t:?} is {:?}", i.value);
            return;
        }

        if i.value == t.0 {
            check.0 = true;
        }
        if i.value == t.1 {
            check.1 = true;
        }

        // println!("Post Ord: >>>>  {:?}", i.value);
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

    let res = lca_iter(&root, (4, 7));
}
