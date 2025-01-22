use tree::Node;

pub fn pre_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
    let mut s = vec![node];

    while let Some(node) = s.pop() {
        print!("{:?} ", node.value);

        if let Some(ref node) = node.right {
            s.push(node);
        }

        if let Some(ref node) = node.left {
            s.push(node);
        }
    }

    println!();
}

fn pre_order_impl<T: std::fmt::Debug>(node: &Node<T>) {
    print!("{:?} ", node.value);

    if let Some(ref node) = node.left {
        pre_order_impl(node);
    }

    if let Some(ref node) = node.right {
        pre_order_impl(node);
    }
}

pub fn pre_order<T: std::fmt::Debug>(node: &Node<T>) {
    pre_order_impl(node);
    println!();
}
