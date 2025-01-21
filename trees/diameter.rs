use tree::Node;

pub fn diameter<T: std::fmt::Display + std::fmt::Debug>(node: &Node<T>, maxi: &mut i32) -> i32 {
    let left = if let Some(ref node) = node.left {
        1 + diameter(node, maxi)
    } else {
        0
    };

    let right = if let Some(ref node) = node.right {
        1 + diameter(node, maxi)
    } else {
        0
    };

    if *maxi < left + right {
        *maxi = left + right;
        println!("maxi updated to {maxi} for node {}", node.value);
    }

    left.max(right)
}
