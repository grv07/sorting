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

// This approch is to keep path in array for a t node in array and then at last
// when we have path for all of them in arrays we traverse through the array and breaks at first path splits.

// Updates path vec with path from node n to target node t.
fn find_path<T>(root: &Node<T>, t: T, path: &mut Vec<T>) -> bool
where
    T: std::cmp::PartialEq + Copy + std::fmt::Debug,
{
    path.push(root.value);

    if root.value == t {
        return true;
    }

    let left = if let Some(node) = &root.left {
        // path.push(node.value);
        find_path(&node, t, path)
    } else {
        false
    };

    let right = if let Some(node) = &root.right {
        // path.push(node.value);
        find_path(&node, t, path)
    } else {
        false
    };

    // println!("{path:?}");

    if left || right {
        return true;
    }
    path.pop();

    false
}

fn path_lca<T>(root: &Node<T>, t: (T, T)) -> Option<T>
where
    T: std::cmp::PartialOrd + Copy + std::fmt::Debug,
{
    let mut p1 = vec![];
    let mut p2 = vec![];

    let t0_path = find_path(root, t.0, &mut p1);
    let t1_path = find_path(root, t.1, &mut p2);

    if !t0_path || !t1_path {
        return None;
    }

    let n = p1.len();
    let mut i = 0;

    // println!(">>> {p1:?} {p2:?}");
    for _ in 0..n {
        if p1[i] != p2[i] {
            break;
        }
        i += 1;
    }

    return Some(p1[i - 1]);
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

    let res = path_lca(&root, (4, 6));
    println!("Path LCA: {:?}", res);

    let res = path_lca(&root, (4, 7));
    println!("Path LCA: {:?}", res);

    let res = path_lca(&root, (5, 7));
    println!("Path LCA: {:?}", res);
}
