use tree::Node;

fn max_width(root: &Node<i32>) -> i32 {
    let mut v = std::collections::VecDeque::new();
    v.push_back((root, 0));

    let mut maxw = 0;

    while !v.is_empty() {
        let width = if let (Some((_, idx1)), Some((_, idx2))) = (v.front(), v.back()) {
            idx2 - idx1 + 1
        } else {
            0
        };

        maxw = maxw.max(width);

        for _ in 0..v.len() {
            if let Some((node, idx)) = v.pop_front() {
                if let Some(node) = &node.left {
                    v.push_back((&node, 2 * idx + 1));
                }

                if let Some(node) = &node.right {
                    v.push_back((&node, 2 * idx + 2));
                }
            }
        }
    }

    maxw
}

pub fn solve() {
    let root = binary_tree!(
        10,
        binary_tree!(2, binary_tree!(20), binary_tree!(1)),
        binary_tree!(
            10,
            right:binary_tree!(-25, binary_tree!(3), binary_tree!(4))
        )
    );
    let res = max_width(&root);
    assert_eq!(res, 4);
}
