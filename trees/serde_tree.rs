use tree::Node;

fn ser(root: &Node<i32>) -> String {
    let mut v = vec![root];
    let mut res = String::new();

    res.push_str(&format!("{},", root.value));
    while let Some(node) = v.pop() {
        match (&node.right, &node.left) {
            (Some(right), Some(left)) => {
                v.push(right);
                v.push(left);
                res.push_str(&format!("{},", left.value));
                res.push_str(&format!("{},", right.value));
            }
            (None, Some(left)) => {
                v.push(left);
                res.push_str(&format!("{},", left.value));
                res.push_str("#,");
            }
            (Some(right), None) => {
                v.push(right);
                res.push_str("#,");
                res.push_str(&format!("{},", right.value));
            }
            _ => {
                res.push_str("#,");
                res.push_str("#,");
            }
        }
    }

    res
}

pub fn solve() {
    let root = binary_tree!(
        1,
        binary_tree!(2),
        binary_tree!(13, binary_tree!(4), binary_tree!(5))
    );

    let res = ser(&root);

    println!("Serialize: {res:?}");
}
