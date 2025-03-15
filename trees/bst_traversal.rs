use tree::Node;

struct TT<'a> {
    v: Vec<(&'a Node<i32>, i32)>,
    ans: Vec<&'a Node<i32>>,
}

impl<'a> TT<'a> {
    fn init(root: &'a Node<i32>) -> TT<'a> {
        Self {
            v: vec![(root, 1)],
            ans: vec![],
        }
    }

    fn next(&mut self) -> Option<&Node<i32>> {
        if !self.ans.is_empty() {
            return self.ans.pop();
        }

        while let Some((node, cnt)) = self.v.pop() {
            if cnt == 2 {
                return Some(node);
            }

            if let Some(right) = &node.right {
                self.v.push((right, 1));
            }

            self.v.push((node, 2));

            if let Some(left) = &node.left {
                self.v.push((left, 1));
            } else {
                match (self.v.pop(), self.v.pop()) {
                    (Some((c, _)), Some((p, _))) => {
                        self.ans.push(p);
                        self.ans.push(c);
                    }
                    (Some((c, _)), _) => {
                        self.ans.push(c);
                    }
                    _ => {}
                }

                return self.ans.pop();
            }
        }

        None
    }

    fn next2(&mut self) -> Option<&Node<i32>> {
        while let Some((node, cnt)) = self.v.pop() {
            if cnt == 2 {
                if let Some(right) = &node.right {
                    self.v.push((right, 1));
                }
                return Some(node);
            }

            self.v.push((node, 2));

            if let Some(left) = &node.left {
                self.v.push((left, 1));
            }
        }

        None
    }
}

pub fn solve() {
    let root = binary_tree!(
        10,
        binary_tree!(
            7,
            binary_tree!(6, left: binary_tree!(5, left: binary_tree!(4))),
            binary_tree!(8)
        ),
        binary_tree!(13, binary_tree!(11), binary_tree!(14))
    );

    let mut tt = TT::init(&root);
    while let Some(res) = tt.next() {
        println!("Next: {:?}", res.value);
    }

    let mut tt = TT::init(&root);
    while let Some(res) = tt.next2() {
        println!("Next: {:?}", res.value);
    }
}
