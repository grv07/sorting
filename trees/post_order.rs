use tree::{print_stack, Node};

fn post_order_impl<T: std::fmt::Debug>(node: &Node<T>) {
    if let Some(ref node) = node.left {
        post_order_impl(node);
    }

    if let Some(ref node) = node.right {
        post_order_impl(node);
    }

    print!("{:?} ", node.value);
}
pub fn post_order<T: std::fmt::Debug>(node: &Node<T>) {
    post_order_impl(node);
    println!();
}

pub fn post_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
    let mut q1 = vec![];
    let mut q2 = vec![node];

    while let Some(item) = q2.pop() {
        q1.push(item);

        if let Some(ref i) = item.left {
            q2.push(i);
        }

        if let Some(ref i) = item.right {
            q2.push(i);
        }
    }

    q1.reverse();
    print_stack(q1);
}
