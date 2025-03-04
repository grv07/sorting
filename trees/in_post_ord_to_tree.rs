use tree::Node;

fn create_tree(in_o: &[i32], post_o: &[i32]) -> Option<Box<Node<i32>>> {
    if in_o.is_empty() && post_o.is_empty() {
        return None;
    }

    let n = post_o.len();
    let mut node = Node::default();

    let root = post_o[n - 1];
    node.value = root;

    if let Some(i) = in_o.iter().position(|x| *x == root) {
        let l_in_o = &in_o[0..i];
        let r_in_o = &in_o[i + 1..];

        let r_pos_o = &post_o[i..n - 1];
        let l_pos_o = &post_o[0..i];

        println!("left {l_in_o:?} right {r_in_o:?}");
        println!("left {l_pos_o:?} right {r_pos_o:?}");

        node.left = create_tree(l_in_o, l_pos_o);
        node.right = create_tree(r_in_o, r_pos_o);
    }

    Some(Box::new(node))
}

pub fn solve() -> Node<i32> {
    let in_o = &[40, 20, 50, 10, 60, 30];
    let post_o = &[40, 50, 20, 60, 30, 10];
    let tree = create_tree(in_o, post_o);
    println!("Tree: {tree:?}");
    *tree.unwrap()
}

// fn main() {}
