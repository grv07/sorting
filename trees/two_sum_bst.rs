use tree::Node;

struct BSTIter<'a> {
    v: Vec<&'a Node<i32>>,
}

impl<'a> BSTIter<'a> {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    fn init_next(&mut self, root: Option<&'a Node<i32>>) {
        self.push_left_most(root);
    }

    fn push_left_most(&mut self, mut root: Option<&'a Node<i32>>) {
        while let Some(node) = root {
            self.v.push(node);
            root = node.left.as_deref();
        }
    }

    fn next(&mut self) -> Option<i32> {
        let node = self.v.pop()?;
        self.push_left_most(node.right.as_deref());
        return Some(node.value);
    }

    fn init_back(&mut self, root: Option<&'a Node<i32>>) {
        self.push_right_most(root);
    }

    fn push_right_most(&mut self, mut root: Option<&'a Node<i32>>) {
        while let Some(node) = root {
            self.v.push(node);
            root = node.right.as_deref();
        }
    }

    fn back(&mut self) -> Option<i32> {
        let node = self.v.pop()?;
        self.push_right_most(node.left.as_deref());
        Some(node.value)
    }
}

fn find_k_sum_pair(mut next: BSTIter, mut back: BSTIter, k: i32) -> bool {
    let mut i = next.next().unwrap();
    let mut j = back.back().unwrap();

    while i < j {
        if i + j == k {
            println!("{i} {j}");
            return true;
        }

        if (i + j) > k {
            j = back.back().unwrap();
        } else {
            i = next.next().unwrap();
        }
    }

    false
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

    let mut ttn = BSTIter::new();
    ttn.init_next(Some(&root));

    let mut ttb = BSTIter::new();
    ttb.init_back(Some(&root));

    let k = 14;
    let res = find_k_sum_pair(ttn, ttb, k);
    println!("Is there any pair for sum {k}, {res:?}");
}
