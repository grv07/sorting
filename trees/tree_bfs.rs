use tree::Node;

pub fn bfs<T: std::fmt::Debug>(node: &Node<T>) {
    let mut q = std::collections::VecDeque::new();
    q.push_back(node);

    while let Some(node) = q.pop_front() {
        print!("{:?} ", node.value);

        if let Some(ref node) = node.left {
            q.push_back(node);
        }

        if let Some(ref node) = node.right {
            q.push_back(node);
        }
    }
    println!();
}
