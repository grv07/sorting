use tree::Node;

// DFS approch for tree traversal
pub fn r_trv<T: Copy + std::fmt::Debug>(root: &Node<T>, res: &mut Vec<Vec<T>>, level: usize) {
    if let Some(i) = res.get_mut(level) {
        if level % 2 == 0 {
            i.insert(0, root.value);
        } else {
            i.push(root.value);
        }
    } else {
        let default = vec![root.value];
        res.push(default);
    }

    if let Some(ref node) = root.left {
        r_trv(node, res, level + 1);
    }

    if let Some(ref node) = root.right {
        r_trv(node, res, level + 1);
    }
}

// BFS approch for tree traversal
pub fn s_trv<T: Copy>(root: &Node<T>, res: &mut Vec<Vec<T>>) {
    let mut queue = vec![(root, 0)];

    while let Some((node, level)) = queue.pop() {
        if let Some(i) = res.get_mut(level) {
            if level % 2 == 0 {
                i.insert(0, node.value);
            } else {
                i.push(node.value);
            }
        } else {
            let default = vec![node.value];
            res.push(default);
        }

        if let Some(ref node) = node.right {
            queue.push((node, level + 1));
        }

        if let Some(ref node) = node.left {
            queue.push((node, level + 1));
        }
    }
}
