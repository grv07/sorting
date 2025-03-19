use std::cmp::max;
use tree::Node;

fn bfs(root: &Node<i32>, mut t: &mut i32) -> i32 {
    let left = if let Some(node) = &root.left {
        max(bfs(node, &mut t), 0)
    } else {
        0
    };

    let right = if let Some(node) = &root.right {
        max(bfs(node, &mut t), 0)
    } else {
        0
    };

    let mut maxi = left + right + root.value;
    // println!("{}", maxi);

    if *t < maxi {
        *t = maxi;
    }

    maxi = maxi.max(0);
    maxi
}

pub fn solve() {
    let root = binary_tree!(
        0,
        binary_tree!(-3, left:binary_tree!(4)),
        binary_tree!(5, binary_tree!(2), binary_tree!(8))
    );

    let mut res = 0;
    bfs(&root, &mut res);
    assert_eq!(res, 16);

    let root = binary_tree!(
        -10,
        binary_tree!(9),
        binary_tree!(20, binary_tree!(15), binary_tree!(7))
    );

    let mut res = 0;
    bfs(&root, &mut res);
    assert_eq!(res, 42);

    let root = binary_tree!(
        10,
        binary_tree!(2, binary_tree!(20), binary_tree!(1)),
        binary_tree!(
            10,
            right:binary_tree!(-25, binary_tree!(3), binary_tree!(4))
        )
    );
    let mut res = 0;
    bfs(&root, &mut res);
    assert_eq!(res, 43);

    let root = binary_tree!(
        1,
        right: binary_tree!(2,  right: binary_tree!(3,  right: binary_tree!(4)))
    );
    let mut res = i32::MIN;
    bfs(&root, &mut res);
    assert_eq!(res, 10);

    let root = binary_tree!(-3, binary_tree!(-2), binary_tree!(-5));
    let mut res = i32::MIN;
    bfs(&root, &mut res);
    assert_eq!(res, -2);

    let root = binary_tree!(5);
    let mut res = i32::MIN;
    bfs(&root, &mut res);
    assert_eq!(res, 5);
}
