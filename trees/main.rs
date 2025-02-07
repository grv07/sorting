// PRE-ORDER: Root-Left-Right
// POST-ORDER: Left-Right-Root
// IN-ORDER: Left-Root-Right

mod all_traversal_in_one;
mod boundry_trv;
mod diameter;
mod identical_tree;
mod inorder;
mod is_tree_balanced;
mod max_depth_tree;
mod max_path;
mod post_order;
mod pre_order;
#[macro_use]
mod tree;
mod k_distance_nodes;
mod lca;
mod root_to_node_path;
mod symmetrical_bst;
mod tree_bfs;
mod vertical_trv;
mod zig_zag_trv;

use all_traversal_in_one::all_in_one_travel;
use boundry_trv::trv_boundry;
use diameter::diameter;
use identical_tree::is_identical;
use inorder::{in_order, in_order_iter};
use is_tree_balanced::is_balance;
// use k_distance_node;
use max_depth_tree::find_maximum_depth;
use max_path::max_path;
use post_order::{post_order, post_order_iter};
use pre_order::{pre_order, pre_order_iter};
pub use tree::{create_tree, Node};
use tree_bfs::bfs;
use vertical_trv::vertical_trv;
use zig_zag_trv::{r_trv, s_trv};

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

fn check_identical() {
    let root = binary_tree!(
        15,
        left: binary_tree!(10, left: binary_tree!(20, binary_tree!(-30), binary_tree!(-15)))
    );

    let root1 = binary_tree!(
        15,
        left: binary_tree!(10, left: binary_tree!(20, binary_tree!(-30), binary_tree!(-15)))
    );

    let res = is_identical(&root, &root1);
    println!("Is identical: {res} \n");
}

fn main() {
    check_identical();

    let root = binary_tree!(
        15,
        left: binary_tree!(10, left: binary_tree!(20, binary_tree!(-30), binary_tree!(-15)))
    );

    let mut res = 0;
    max_path(&root, &mut res);
    println!("Max path in tree between teo nodes: {res} \n");

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
        println!("Tree is not a balanced tree\n");
    } else {
        println!("Tree is a balanced tree\n");
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

    let mut res = vec![];
    r_trv(&root, &mut res, 0);
    println!("{res:?}");

    let mut res = vec![];
    s_trv(&root, &mut res);
    println!("{res:?}");

    let mut res = vec![vec![]; 3];
    trv_boundry(&root, &mut res);
    println!("{res:?}");

    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(4, right: binary_tree!(5, right: binary_tree!(6))),
            binary_tree!(7)
        ),
        binary_tree!(3, binary_tree!(9), binary_tree!(10))
    );

    let mut res = vec![vec![]; 2];
    vertical_trv(&root, &mut res);
    res[0].reverse();
    println!("{res:?}");

    symmetrical_bst::solve();

    root_to_node_path::solve();
    lca::solve();

    k_distance_nodes::solve();
}
