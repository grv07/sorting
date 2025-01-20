use tree::Node;

pub fn is_balance<T: std::fmt::Debug + std::fmt::Display>(node: &Node<T>) -> i32 {
    let left = if let Some(ref node) = node.left {
        is_balance(node)
    } else {
        0
    };

    if left == -1 {
        return -1;
    }

    let right = if let Some(ref node) = node.right {
        is_balance(node)
    } else {
        0
    };

    if right == -1 {
        return -1;
    }

    println!("{left} {right} for node {}", node.value);

    if (left - right).abs() > 1 {
        return -1;
    }

    left.max(right) + 1
}
