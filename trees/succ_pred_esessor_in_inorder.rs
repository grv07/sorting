use tree::Node;

fn inorder(root: &Node<i32>, k: i32) -> i32 {
    let mut v = vec![(root, 1)];

    while let Some((node, cnt)) = v.pop() {
        if cnt == 2 {
            if node.value > k {
                return node.value;
            }
            continue;
        }

        if k >= node.value {
            if let Some(right) = &node.right {
                v.push((right, 1));
            }
        }

        v.push((node, 2));

        if k < node.value {
            if let Some(left) = &node.left {
                v.push((left, 1));
            }
        }
    }

    -1
}

fn successor<T>(root: &Node<T>, k: T) -> Option<T>
where
    T: std::cmp::PartialOrd + std::fmt::Debug + Copy,
{
    let mut root = Some(root);

    let mut succ = None;
    while let Some(node) = root {
        if k >= node.value {
            root = node.right.as_deref();
        } else {
            succ = Some(node.value);
            root = node.left.as_deref();
        }
    }

    succ
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

    assert_eq!(inorder(&root, 10), 11);
    assert_eq!(inorder(&root, 7), 8);
    assert_eq!(inorder(&root, 13), 14);
    assert_eq!(inorder(&root, 11), 13);
    assert_eq!(inorder(&root, 4), 5);
    // If that node value not in tree
    assert_eq!(inorder(&root, 3), 4);
    assert_eq!(inorder(&root, 1), 4);

    assert_eq!(successor(&root, 3), Some(4));
    assert_eq!(successor(&root, 10), Some(11));
    assert_eq!(successor(&root, 7), Some(8));
    assert_eq!(successor(&root, 13), Some(14));
    assert_eq!(successor(&root, 11), Some(13));
    assert_eq!(successor(&root, 4), Some(5));
    // If that node value not in tree
    assert_eq!(successor(&root, 3), Some(4));
    assert_eq!(successor(&root, 1), Some(4));
}
