use std::default::Default;

#[derive(Default, Debug)]
pub struct Node<T> {
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T,
}

impl<T: Default> Node<T> {
    pub fn new(value: T, left: Option<Node<T>>, right: Option<Node<T>>) -> Self {
        let mut node = Self {
            value,
            ..Default::default()
        };
        node.set_left(left);
        node.set_right(right);

        node
    }

    pub fn set_left(&mut self, left: Option<Node<T>>) {
        self.left = left.map(|left| Box::new(left));
    }

    pub fn set_right(&mut self, right: Option<Node<T>>) {
        self.right = right.map(|right| Box::new(right));
    }
}

pub fn create_tree(value: i32) -> Node<i32> {
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

// fn create_tree1(value: i32) -> Node<i32> {
//     let mut root: Node<i32> = Node::new(value, None, None);

//     root.set_left(Some(Node::new(
//         2,
//         Some(Node::new(1, None, None)),
//         Some(Node::new(3, None, None)),
//     )));

//     root.set_right(Some(Node::new(5, None, None)));

//     root
// }
