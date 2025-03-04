use tree::Node;

fn create_tree(ino: &[i32], preo: &[i32]) -> Option<Box<Node<i32>>> {
    if ino.len() == 0 && preo.len() == 0 {
        return None;
    }

    let root = preo[0];
    let mut node = Node::new(root, None, None);
    // println!("Root: {root}");
    if let Some(i) = ino.iter().position(|x| *x == root) {
        let ino_l = &ino[..i];
        let ino_r = &ino[(i + 1)..];

        let preo_l = &preo[1..i + 1];
        let preo_r = &preo[i + 1..];

        // println!("{ino_l:?} {preo_l:?}");
        // println!("{ino_r:?} {preo_r:?}");

        node.left = create_tree(ino_l, preo_l);
        node.right = create_tree(ino_r, preo_r);
    }

    Some(Box::new(node))
}

pub fn solve() -> Node<i32> {
    //   TREE
    //           10
    //       20     30
    //     40  50  60
    let preo = &[40, 20, 50, 10, 60, 30];
    let ino = &[10, 20, 40, 50, 30, 60];

    let tree = create_tree(preo, ino);
    *tree.unwrap()
}
