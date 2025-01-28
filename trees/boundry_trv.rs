use tree::Node;

enum NType<'a, T: Copy> {
    Left(&'a Node<T>),
    Right(&'a Node<T>),
    Leaf(&'a Node<T>),
}

pub fn trv_boundry<T: Copy>(root: &Node<T>, res: &mut Vec<Vec<T>>) {
    let mut s: Vec<NType<T>> = vec![];

    res[0].push(root.value);

    if let Some(ref node) = root.right {
        s.push(NType::Right(&*node));
        res[2].push(node.value);
    }

    if let Some(ref node) = root.left {
        s.push(NType::Left(&*node));
        res[0].push(node.value);
    }

    while let Some(ref node) = s.pop() {
        match node {
            NType::Left(node) => match (node.left.as_ref(), node.right.as_ref()) {
                (Some(ref left), _) => {
                    res[0].push(left.value);
                    s.push(NType::Left(&*left));
                }
                (None, Some(ref right)) => {
                    s.push(NType::Left(&*right));
                    res[0].push(right.value);
                }
                (None, None) => {
                    res[0].pop();
                    res[1].push(node.value);
                }
            },

            NType::Right(node) => match (node.left.as_ref(), node.right.as_ref()) {
                (_, Some(ref right)) => {
                    s.push(NType::Right(&*right));
                    res[2].push(right.value);
                }
                (Some(ref left), None) => {
                    s.push(NType::Right(&left));
                    res[2].push(left.value);
                }
                (None, None) => {
                    res[2].pop();
                    res[2].reverse();
                    res[1].push(node.value);
                }
            },
            NType::Leaf(_node) => {}
        }
    }
}
