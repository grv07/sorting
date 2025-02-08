use std::collections::HashMap;
use tree::Node;

fn get_k_nodes<T>(node: &Node<T>, k: i32, ans: &mut Vec<T>)
where
    T: Copy,
{
    if k == 0 {
        ans.push(node.value);
        return;
    }

    if let Some(ref node) = node.left {
        get_k_nodes(node, k - 1, ans)
    }

    if let Some(ref node) = node.right {
        get_k_nodes(node, k - 1, ans)
    }
}

fn find_nodes<T>(root: &Node<T>, t: T, k: i32, ans: &mut Vec<T>) -> i32
where
    T: Copy + std::cmp::PartialEq + std::fmt::Debug,
{
    if root.value == t {
        println!("Target found {t:?}");
        println!("get all d {k} from {:?}", root.value);
        get_k_nodes(root, k, ans);

        return 1;
    }

    let left = if let Some(ref node) = root.left {
        find_nodes(node, t, k, ans)
    } else {
        -1
    };

    if left != -1 {
        println!("Left is not -1 for {:?}", root.value);
        // if let Some(ref node) = root.left {}
        if k - left == 0 {
            ans.push(root.value);
        }
        if let Some(ref node) = root.right {
            get_k_nodes(node, k - (left + 1), ans);
        }
        return left + 1;
    }

    let right = if let Some(ref node) = root.right {
        find_nodes(node, t, k, ans)
    } else {
        -1
    };

    if right != -1 {
        // if let Some(ref node) = root.right {}
        if k - right == 0 {
            ans.push(root.value);
        }
        if let Some(ref node) = root.left {
            get_k_nodes(node, k - (right + 1), ans);
        }
        return right + 1;
    }

    if k == 0 {
        ans.push(root.value);
    }

    -1
}

fn find_target<T>(root: &Node<T>, t: T) -> Option<Node<T>>
where
    T: std::cmp::PartialEq + Clone,
{
    let mut q = vec![root];

    while let Some(node) = q.pop() {
        if node.value == t {
            return Some((*node).clone());
        }

        if let Some(ref node) = node.right {
            q.push(node);
        }
        if let Some(ref node) = node.left {
            q.push(node);
        }
    }
    None
}

fn parent_map<T>(root: &Node<T>) -> HashMap<&Node<T>, &Node<T>>
where
    T: std::hash::Hash + Eq,
{
    let mut map: HashMap<&Node<T>, &Node<T>> = HashMap::new();
    let mut q = vec![root];

    while let Some(parent) = q.pop() {
        if let Some(ref node) = parent.right {
            q.push(node);
            map.insert(node, parent);
        }
        if let Some(ref node) = parent.left {
            q.push(node);
            map.insert(node, parent);
        }
    }

    map
}

fn dfs<T>(
    root: &Node<T>,
    prev: &Node<T>,
    k: i32,
    ans: &mut Vec<T>,
    map: &HashMap<&Node<T>, &Node<T>>,
) where
    T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + Clone,
{
    if k == 0 {
        ans.push(root.value.clone());
        return;
    }

    if let Some(ref node) = root.left {
        if *prev != **node {
            dfs(node, root, k - 1, ans, map)
        }
    }

    if let Some(ref node) = root.right {
        if *prev != **node {
            dfs(node, root, k - 1, ans, map)
        }
    }

    if let Some(parent) = map.get(root) {
        if *parent != prev {
            dfs(parent, root, k - 1, ans, map)
        }
    }
}

pub fn solve() {
    //       1
    //     2   6
    //   3   5
    //  4
    // 7
    let root = binary_tree!(
        1,
        binary_tree!(
            2,
            binary_tree!(3, left: binary_tree!(4,left: binary_tree!(7))),
            binary_tree!(5)
        ),
        binary_tree!(6)
    );
    let mut ans = vec![];

    let t = 4;
    let k = 3;

    find_nodes(&root, t, k, &mut ans);
    println!("Ans: {ans:?}");

    let t = find_target(&root, t);

    let map = parent_map(&root);

    if let Some(ref node) = t {
        let mut ans = vec![];
        dfs(node, node, 3, &mut ans, &map);
        println!("Ans is: {ans:?}");
    }
}
