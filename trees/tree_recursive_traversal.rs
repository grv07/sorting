// PRE-ORDER: Root-Left-Right
// POST-ORDER: Left-Right-Root
// IN-ORDER: Left-Root-Right

mod all_traversal_in_one;
mod diameter;
mod inorder;
mod is_tree_balanced;
mod max_depth_tree;
mod max_path;
mod post_order;
mod pre_order;
mod tree;
mod tree_bfs;

use all_traversal_in_one::all_in_one_travel;
use diameter::diameter;
use inorder::{in_order, in_order_iter};
use is_tree_balanced::is_balance;
use max_depth_tree::find_maximum_depth;
use max_path::max_path;
use post_order::{post_order, post_order_iter};
use pre_order::{pre_order, pre_order_iter};
use tree::{create_tree, Node};
use tree_bfs::bfs;

fn _print_options_stack<T: std::fmt::Debug>(v: &Vec<Option<&Node<T>>>) {
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

fn main() {
    let root: Node<i32> = create_tree(1);

    let mut res = 0;
    max_path(&root, &mut res);
    println!("Result: {res}");

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
