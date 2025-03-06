use tree::Node;

fn ser(root: &Node<i32>) -> String {
    let mut v = vec![root];
    let mut res = String::new();

    res.push_str(&format!("{},", root.value));
    while let Some(node) = v.pop() {
        match (&node.right, &node.left) {
            (Some(right), Some(left)) => {
                v.push(right);
                v.push(left);
                res.push_str(&format!("{},", left.value));
                res.push_str(&format!("{},", right.value));
            }
            (None, Some(left)) => {
                v.push(left);
                res.push_str(&format!("{},", left.value));
                res.push_str("#,");
            }
            (Some(right), None) => {
                v.push(right);
                res.push_str("#,");
                res.push_str(&format!("{},", right.value));
            }
            _ => {
                res.push_str("#,");
                res.push_str("#,");
            }
        }
    }

    res
}

fn deser(mut a: Vec<&str>) -> Option<Node<&str>> {
    let mut root = if let Some(root) = a.pop() {
        Node::new(root, None, None)
    } else {
        return None;
    };

    let mut v = vec![&mut root];

    while let Some(node) = v.pop() {
        if let (Some(left), Some(right)) = (a.pop(), a.pop()) {
            if right != "#" {
                node.right = Some(Box::new(Node::new(right, None, None)));
                let rr = node.right.as_mut().unwrap();
                v.push(rr);
            }

            if left != "#" {
                node.left = Some(Box::new(Node::new(left, None, None)));
                let rr = node.left.as_mut().unwrap();
                v.push(rr);
            }
        }
    }

    Some(root)
}

pub fn solve() -> Option<Node<&'static str>> {
    let root = binary_tree!(
        1,
        binary_tree!(2),
        binary_tree!(13, binary_tree!(4), binary_tree!(5))
    );

    let res = ser(&root);

    println!("Serialize: {res:?}");

    // let tree = deser(vec!["1", "2", "13", "#", "#", "4", "5", "#", "#", "#", "#"]);
    let tree = deser(vec!["#", "#", "#", "#", "5", "4", "#", "#", "13", "2", "1"]);
    println!("Serialize: {tree:?}");
    tree
}
