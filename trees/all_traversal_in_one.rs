use tree::{print_stack, Node};

enum TT {
    InOrder = 1,
    PreOrder = 2,
    PostOrder = 3,
}

pub fn all_in_one_travel<T: std::fmt::Display + std::fmt::Debug>(node: &Node<T>) {
    let mut pre_ord = vec![];
    let mut in_ord = vec![];
    let mut post_ord = vec![];

    let mut q = vec![(node, TT::PreOrder)];

    while let Some((node, tt)) = q.pop() {
        match tt {
            TT::PreOrder => {
                pre_ord.push(node);
                q.push((node, TT::InOrder));
                if let Some(ref n) = node.left {
                    q.push((n, TT::PreOrder));
                }
            }
            TT::InOrder => {
                in_ord.push(node);
                q.push((node, TT::PostOrder));
                if let Some(ref n) = node.right {
                    q.push((n, TT::PreOrder));
                }
            }
            TT::PostOrder => {
                post_ord.push(node);
            }
        }
    }

    print!("Pre  ");
    print_stack(pre_ord);
    print!("Post ");
    print_stack(post_ord);
    print!("In   ");
    print_stack(in_ord);
}
