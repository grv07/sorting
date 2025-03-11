use tree::Node;

fn search<T>(root: &Node<T>, k: T) -> Option<&Node<T>>
where
    T: Copy,
    T: std::cmp::PartialOrd,
    T: std::cmp::PartialEq,
{
    if root.value == k {
        return Some(root);
    }

    if root.value < k {
        if let Some(right) = &root.right {
            return search(right, k);
        }
    }

    if root.value > k {
        if let Some(left) = &root.left {
            return search(left, k);
        }
    }

    None
}

fn search_s<T>(root: &Node<T>, k: T) -> Option<&Node<T>>
where
    T: Copy,
    T: std::cmp::PartialOrd,
    T: std::cmp::PartialEq,
{
    let mut v = vec![root];

    while let Some(node) = v.pop() {
        if node.value == k {
            return Some(node);
        }

        if node.value < k {
            if let Some(right) = &node.right {
                v.push(right);
            }
        }

        if node.value > k {
            if let Some(left) = &node.left {
                v.push(left);
            }
        }
    }

    None
}

fn ceil<T>(root: &Node<T>, k: T) -> i32
where
    T: Copy,
    T: Into<i32>,
    T: std::ops::Sub<Output = T>,
    T: std::cmp::PartialOrd,
    T: std::cmp::PartialEq,
{
    // find v >= k where v is possible smallest
    let mut ceil = -1;

    let mut root = Some(root);

    while let Some(node) = root {
        if node.value == k {
            return node.value.into();
        }

        if node.value < k {
            root = node.right.as_deref();
        } else {
            ceil = node.value.into();
            root = node.left.as_deref();
        }
    }

    ceil
}

fn floor<T>(root: &Node<T>, k: T) -> i32
where
    T: Copy,
    T: Into<i32>,
    T: std::ops::Sub<Output = T>,
    T: std::cmp::PartialOrd,
    T: std::cmp::PartialEq,
{
    // find v <= k where v is possible smallest
    let mut floor = -1;

    let mut root = Some(root);

    while let Some(node) = root {
        if node.value == k {
            return node.value.into();
        }

        if node.value > k {
            root = node.left.as_deref();
        } else {
            floor = node.value.into();
            root = node.right.as_deref();
        }
    }

    floor
}

pub fn solve() {
    let root = binary_tree!(
        10,
        binary_tree!(
            7,
            binary_tree!(6, left: binary_tree!(5, left: binary_tree!(4))),
            binary_tree!(8)
        ),
        binary_tree!(13, binary_tree!(11), binary_tree!(14))
    );

    let res = search(&root, 11);
    println!("Search: {res:?}");

    let res = search_s(&root, 14);
    println!("Search Stack: {res:?}");

    let res = ceil(&root, 12);
    println!("Search ceil: {res:?}");

    let res = floor(&root, 29);
    println!("Search floor: {res:?}");
}
