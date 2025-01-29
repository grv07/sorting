use tree::Node;

pub fn vertical_trv<T>(root: &Node<T>, res: &mut Vec<Vec<Vec<T>>>)
where
    T: std::cmp::PartialOrd + std::ops::Neg<Output = T> + Copy,
{
    let mut q: Vec<(&Node<T>, i32)> = vec![(root, 0)];

    while let Some((node, level)) = q.pop() {
        let idx = level.abs() as usize;

        let (res, idx) = if level < 0 {
            (&mut res[0], idx - 1)
        } else {
            (&mut res[1], idx)
        };

        match res.get_mut(idx) {
            Some(inner) => inner.push(node.value),
            None => {
                res.insert(idx, vec![node.value]);
            }
        }

        if let Some(ref node) = node.right {
            q.push((node, level + 1))
        }

        if let Some(ref node) = node.left {
            q.push((node, level - 1))
        }
    }
}
