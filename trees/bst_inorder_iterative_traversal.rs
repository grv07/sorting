use tree::Node;

struct TT<'a> {
    v: Vec<&'a Node<i32>>,
}

impl<'a> TT<'a> {
    fn init(root: Option<&'a Node<i32>>) -> TT<'a> {
        let mut iter = Self { v: vec![] };
        iter.push_leftmost(root);
        iter
    }

    fn push_leftmost(&mut self, mut root: Option<&'a Node<i32>>) {
        while let Some(node) = root {
            self.v.push(node);
            root = node.left.as_deref();
        }
    }

    fn next(&mut self) -> Option<i32> {
        let node = self.v.pop()?;

        self.push_leftmost(node.right.as_deref());

        Some(node.value)
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

    let mut tt = TT::init(Some(&root));
    while let Some(res) = tt.next() {
        println!("Next: {:?}", res);
    }
}
