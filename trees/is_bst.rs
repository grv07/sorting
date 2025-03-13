use tree::Node;

fn is_bst(root: &Node<i32>) -> bool {
    let mut v = vec![(root, i32::MIN..i32::MAX)];

    while let Some((node, range)) = v.pop() {
        // println!("{:?} {range:?}", node.value);
        if !range.contains(&node.value) {
            // println!("fails");
            return false;
        }

        if let Some(left) = &node.left {
            v.push((left, range.start..node.value));
        }

        if let Some(right) = &node.right {
            v.push((right, (node.value + 1)..range.end));
        }
    }

    true
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

    let res = is_bst(&root);
    println!("Res: {res}");
}
