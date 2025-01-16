// PRE-ORDER: Root-Left-Right
// POST-ORDER: Left-Right-Root
// IN-ORDER: Left-Root-Right

use std::default::Default;

#[derive(Default, Debug)]
struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T: Default> Node<T> {
    fn new(value: T, left: Option<Node<T>>, right: Option<Node<T>>) -> Self {
        let mut node = Self {
            value,
            ..Default::default()
        };
        node.set_left(left);
        node.set_right(right);

        node
    }

    fn set_left(&mut self, left: Option<Node<T>>) {
        self.left = left.map(|left| Box::new(left));
    }

    fn set_right(&mut self, right: Option<Node<T>>) {
        self.right = right.map(|right| Box::new(right));
    }
}

fn create_tree(value: i32) -> Node<i32> {
    let mut root: Node<i32> = Node::new(value, None, None);

    root.set_left(Some(Node::new(
        2,
        Some(Node::new(4, None, None)),
        Some(Node::new(5, Some(Node::new(6, None, None)), None)),
    )));

    root.set_right(Some(Node::new(
        3,
        Some(Node::new(7, None, None)),
        Some(Node::new(
            8,
            Some(Node::new(9, None, None)),
            Some(Node::new(10, None, None)),
        )),
    )));

    root
}

fn create_tree1(value: i32) -> Node<i32> {
    let mut root: Node<i32> = Node::new(value, None, None);

    root.set_left(Some(Node::new(
        2,
        Some(Node::new(1, None, None)),
        Some(Node::new(3, None, None)),
    )));

    root.set_right(Some(Node::new(5, None, None)));

    root
}

fn in_order<T: std::fmt::Debug>(node: &Node<T>) {
    if let Some(ref node) = node.left {
        in_order(node);
    }

    print!("{:?} ", node.value);

    if let Some(ref node) = node.right {
        in_order(node);
    }
}

fn pre_order<T: std::fmt::Debug>(node: &Node<T>) {
    print!("{:?} ", node.value);

    if let Some(ref node) = node.left {
        pre_order(node);
    }

    if let Some(ref node) = node.right {
        pre_order(node);
    }
}

fn post_order<T: std::fmt::Debug>(node: &Node<T>) {
    if let Some(ref node) = node.left {
        post_order(node);
    }

    if let Some(ref node) = node.right {
        post_order(node);
    }

    print!("{:?} ", node.value);
}

fn main() {
    let root: Node<i32> = create_tree(1);
    in_order(&root);
    println!();
    pre_order(&root);
    println!();
    post_order(&root);

    println!();

    let root: Node<i32> = create_tree1(4);
    in_order(&root);
    println!();
    pre_order(&root);
    println!();
    post_order(&root);
}
