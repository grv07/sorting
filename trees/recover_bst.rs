// We have a invalid BST.
// Write a code to modify this to a valid tree.

use tree::Node;

fn inorder<T>(root: &Node<T>) -> Vec<T>
where
    T: Copy + std::fmt::Debug + std::cmp::PartialOrd,
{
    let mut v = vec![];
    let mut res = vec![];
    let mut prev: Option<&Node<T>> = None;
    let mut first = None;
    let mut last = None;

    let mut curr = Some(root);

    while curr.is_some() || !v.is_empty() {
        while let Some(node) = curr {
            curr = node.left.as_deref();
            v.push(node);
        }

        if let Some(node) = v.pop() {
            if let Some(prev) = prev {
                if prev.value > node.value {
                    if first.is_none() {
                        first = Some(prev.value);
                    }
                    last = Some(node.value);
                }
            }

            res.push(node.value);
            prev = Some(node);
            curr = node.right.as_deref();
        }
    }

    println!("first: {first:?} last:{last:?}");

    res
}

fn recover_bst<T>(root: &Node<T>)
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + Copy,
{
    let inord = inorder(root);
    println!("Inorder: {inord:?}");
    for i in 1..inord.len() {
        if inord[i - 1] > inord[i] {
            println!("Is invalid tree bcs of {:?} {:?}", inord[i - 1], inord[i]);
            return;
        }
    }
}

pub fn solve() {
    let root = binary_tree!(
        10,
        binary_tree!(
            6,
            binary_tree!(7, left: binary_tree!(5, left: binary_tree!(4))),
            binary_tree!(8)
        ),
        binary_tree!(13, binary_tree!(11), binary_tree!(14))
    );
    recover_bst(&root);
}
