use std::collections::VecDeque;
use tree::Node;

pub fn find_maximum_depth<T: std::fmt::Debug>(node: &Node<T>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back(node);

    let mut cnt = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            if let Some(node) = q.pop_front() {
                if let Some(ref right) = node.right {
                    q.push_back(right);
                }
                if let Some(ref left) = node.left {
                    q.push_back(left);
                }
            }
        }
        cnt += 1;
    }
    cnt
}
