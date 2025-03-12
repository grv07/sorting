use tree::Node;

fn lca(root: &Node<i32>, q: (i32, i32)) -> i32 {
    let mut v = vec![root];

    while let Some(node) = v.pop() {
        if node.value == q.0 || node.value == q.1 {
            return node.value;
        }

        if (node.value > q.0 && node.value < q.1) || (node.value < q.0 && node.value > q.1) {
            return node.value;
        }

        if node.value > q.0 && node.value > q.1 {
            if let Some(left) = &node.left {
                v.push(left);
            }
        }

        if node.value < q.0 && node.value < q.1 {
            if let Some(right) = &node.right {
                v.push(right);
            }
        }
    }
    -1
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

    let res = lca(&root, (11, 6));
    println!("Res: {res:?}");
}
