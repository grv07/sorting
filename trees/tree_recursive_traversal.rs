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

fn print_stack<T: std::fmt::Debug>(v: &Vec<&Node<T>>) {
    print!("Stack: ");
    for i in v {
        print!("{:?} ", i.value);
    }
    println!();
}

fn in_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
    let mut s = vec![Some(node), Some(node)];

    let mut cnt = 0;
    while let Some(node) = s.pop() {
        if node.is_none() {
            cnt += 1;
            if cnt > 10 {
                break;
            }
            while let Some(Some(node)) = s.pop() {
                // print_stack(&s);
                print!("{:?} ", node.value);
                if let Some(ref node) = node.right {
                    // println!("Pushing the right");
                    s.push(Some(node));
                    // print_stack(&s);
                }
            }
        }

        let mut nn = node;
        while let Some(ref n) = nn {
            // if let Some(ref n) = nn {
            cnt += 1;
            if cnt > 10 {
                break;
            }

            s.push(n.left.as_deref());
            // println!("Pushing left {:?}", n.left);
            nn = n.left.as_deref();
            // print_stack(&s);
        }
    }
}

fn post_order_iter<T: std::fmt::Debug>(node: &Node<T>) {
    let mut q1 = vec![node];
    let mut q2 = vec![node];

    while let Some(item) = q2.pop() {
        // q1.push(item);

        if let Some(ref i) = item.right {
            q2.push(i);
            q1.push(i);
        }

        if let Some(ref i) = item.left {
            q2.push(i);
            q1.push(i);
        }
    }

    q1.reverse();
    print_stack(&q1);
    // println!("{q1:?}");
}

fn main() {
    let root: Node<i32> = create_tree(1);
    in_order(&root);
    println!();
    pre_order(&root);
    println!();
    post_order(&root);

    println!();
    println!();
    println!("In order: ");
    let root: Node<i32> = create_tree1(4);
    in_order(&root);
    println!();
    in_order_iter(&root);

    println!();
    println!();
    println!("Pre order: ");
    pre_order(&root);
    println!();
    pre_order_iter(&root);
    println!();

    println!("Post order: ");
    post_order(&root);
    println!();
    post_order_iter(&root);

    println!();
    println!("BFS: ");
    bfs(&root);
}
