use tree::Node;

fn in_order_impl<T: std::fmt::Debug>(node: &Node<T>) {
    if let Some(ref node) = node.left {
        in_order_impl(node);
    }

    print!("{:?} ", node.value);

    if let Some(ref node) = node.right {
        in_order_impl(node);
    }
}

pub fn in_order<T: std::fmt::Debug>(node: &Node<T>) {
    in_order_impl(node);
    println!();
}

pub fn in_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
    let mut s = vec![Some(node), Some(node)];

    while let Some(node) = s.pop() {
        if node.is_none() {
            while let Some(Some(node)) = s.pop() {
                print!("{:?} ", node.value);
                if let Some(ref node) = node.right {
                    // println!("Pushing the right: ");
                    s.push(Some(node));
                    s.push(Some(node));
                    // print_options_stack(&s);
                    break;
                }
            }
        }
        let mut nn = node;
        while let Some(ref n) = nn {
            s.push(n.left.as_deref());
            nn = n.left.as_deref();
        }
    }
    println!();
}
