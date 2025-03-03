use tree::Node;

fn left_height<T>(mut node: &Node<T>) -> i32 {
    let mut h = 0;

    while let Some(ref left) = node.left {
        h += 1;
        node = left;
    }

    h
}

fn right_height<T>(mut node: &Node<T>) -> i32 {
    let mut h = 0;

    while let Some(ref right) = node.right {
        h += 1;
        node = right;
    }

    h
}

fn height<T>(node: &Node<T>) -> i32 {
    let lh = left_height(node);
    let rh = right_height(node);

    // println!("lh: {lh} rh: {rh}");
    if rh == lh {
        let h = (2 << lh) - 1;
        // println!("lh and rh is same {lh} {}", h);
        return h;
    } else {
        let lh = if let Some(ref left) = node.left {
            height(left)
        } else {
            0
        };

        let rh = if let Some(ref right) = node.right {
            height(right)
        } else {
            0
        };

        return lh + rh + 1;
    }
}

pub fn solve() {
    //  TREE
    //     1
    //   2   3
    //  4 5 6 7
    // 8 9
    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(4, binary_tree!(8), binary_tree!(9)),
            binary_tree!(5)
        ),
        binary_tree!(3, binary_tree!(6), binary_tree!(7))
    );

    println!("");

    let res = height(&root);
    println!("Res: {res}");
}
