use tree::Node;

fn get_path<T>(root: &Node<T>, res: &mut Vec<T>, t: T) -> bool
where
    T: std::fmt::Debug + std::cmp::PartialEq + Copy,
{
    res.push(root.value);

    if root.value == t {
        return true;
    }

    if let Some(ref node) = root.left {
        return get_path(node, res, t);
    }

    if let Some(ref node) = root.right {
        return get_path(node, res, t);
    }

    // backtracking to pop the last item
    res.pop();

    false
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

    let mut res = vec![];
    get_path(&root, &mut res, 6);

    println!("{res:?}");
}
