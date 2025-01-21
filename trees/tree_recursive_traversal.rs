// PRE-ORDER: Root-Left-Right
// POST-ORDER: Left-Right-Root
// IN-ORDER: Left-Root-Right

mod diameter;
mod inorder;
mod is_tree_balanced;
mod tree;

use diameter::diameter;
use inorder::{in_order, in_order_iter, print_stack};
use is_tree_balanced::is_balance;
use std::collections::VecDeque;
use tree::{create_tree, Node};

fn pre_order_impl<T: std::fmt::Debug>(node: &Node<T>) {
    print!("{:?} ", node.value);

    if let Some(ref node) = node.left {
        pre_order_impl(node);
    }

    if let Some(ref node) = node.right {
        pre_order_impl(node);
    }
}
fn pre_order<T: std::fmt::Debug>(node: &Node<T>) {
    pre_order_impl(node);
    println!();
}

fn post_order_impl<T: std::fmt::Debug>(node: &Node<T>) {
    if let Some(ref node) = node.left {
        post_order_impl(node);
    }

    if let Some(ref node) = node.right {
        post_order_impl(node);
    }

    print!("{:?} ", node.value);
}
fn post_order<T: std::fmt::Debug>(node: &Node<T>) {
    post_order_impl(node);
    println!();
}

fn bfs<T: std::fmt::Debug>(node: &Node<T>) {
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

fn pre_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
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

fn print_options_stack<T: std::fmt::Debug>(v: &Vec<Option<&Node<T>>>) {
    print!("Stack: ");
    for i in v {
        if i.is_none() {
            print!(" None ");
        } else {
            print!(" {:?} ", i.unwrap().value);
        }
    }
    println!();
}

fn post_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
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

enum TT {
    InOrder = 1,
    PreOrder = 2,
    PostOrder = 3,
}

fn all_in_one_travel<T: std::fmt::Display + std::fmt::Debug>(node: &Node<T>) {
    let mut pre_ord = vec![];
    let mut in_ord = vec![];
    let mut post_ord = vec![];

    let mut q = vec![(node, TT::PreOrder)];

    while let Some((node, tt)) = q.pop() {
        match tt {
            TT::PreOrder => {
                pre_ord.push(node);
                q.push((node, TT::InOrder));
                if let Some(ref n) = node.left {
                    q.push((n, TT::PreOrder));
                }
            }
            TT::InOrder => {
                in_ord.push(node);
                q.push((node, TT::PostOrder));
                if let Some(ref n) = node.right {
                    q.push((n, TT::PreOrder));
                }
            }
            TT::PostOrder => {
                post_ord.push(node);
            }
        }
    }

    print!("Pre  ");
    print_stack(pre_ord);
    print!("Post ");
    print_stack(post_ord);
    print!("In   ");
    print_stack(in_ord);
}

fn find_maximum_depth<T: std::fmt::Debug>(node: &Node<T>) -> i32 {
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

fn main() {
    let root: Node<i32> = create_tree(1);

    println!("In order");
    print!("REC ");
    in_order(&root);
    print!("ITR ");
    in_order_iter(&root);

    println!("\nPre order ");
    print!("REC ");
    pre_order(&root);
    print!("ITR ");
    pre_order_iter(&root);

    println!("\nPost order: ");
    print!("REC ");
    post_order(&root);
    print!("ITR ");
    post_order_iter(&root);

    println!("\nMax depth is: {}", find_maximum_depth(&root));

    all_in_one_travel(&root);

    println!("\nBFS: ");
    bfs(&root);

    if is_balance(&root) == -1 {
        println!("Tree is not a balanced tree");
    } else {
        println!("Tree is a balanced tree");
    }

    let root = binary_tree!(
        1,
        binary_tree!(2, left: binary_tree!(10, left: binary_tree!(11))),
        binary_tree!(
            3,
            binary_tree!(4, left: binary_tree!(5, left: binary_tree!(6))),
            binary_tree!(7, left: binary_tree!(8, left: binary_tree!(9)))
        )
    );

    let mut res = 0;
    diameter(&root, &mut res);
    println!("{res}");
}

#[macro_export]
macro_rules! binary_tree {
    // Base case: Leaf Node with no children
    ($value:expr) => {
        Node::new($value, None, None)
    };

    // Recurssive Case: Node with children.
    ($value: expr, $left: expr, $right: expr) => {
        Node {
            value: $value,
            left: Some(Box::new($left)),
            right: Some(Box::new($right)),
        }
    };

    // Recurssive Case: Node with children.
    ($value: expr, left: $left:expr) => {
        Node {
            value: $value,
            left: Some(Box::new($left)),
            right: None,
        }
    };

    // Recurssive Case: Node with children.
    ($value: expr, right: $right:expr) => {
        Node {
            value: $value,
            left: None,
            right: Some(Box::new($right)),
        }
    };
}
