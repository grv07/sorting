use tree::Node;

pub fn max_path<T: Into<i32> + Copy>(node: &Node<T>, maxi: &mut i32) -> i32 {
    let left = if let Some(ref node) = node.left {
        max_path(node, maxi)
    } else {
        0
    };

    let right = if let Some(ref node) = node.right {
        max_path(node, maxi)
    } else {
        0
    };

    *maxi = *maxi.max(&mut (left + right + node.value.into()));

    left.max(right) + node.value.into()
}
