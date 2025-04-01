use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    childs: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    // fn new(c: char, is_end: bool) -> Self {
    //     let mut childs = HashMap::new();
    //     childs.insert(c, TrieNode::default());

    //     Self { childs, is_end }
    // }

    fn empty(is_end: bool) -> Self {
        Self {
            is_end,
            ..Default::default()
        }
    }

    fn find(&self, q: &str) -> bool {
        let mut n = q.len();
        let mut root = self;

        for c in q.chars() {
            n -= 1;

            if let Some(node) = root.childs.get(&c) {
                root = node;
                if n <= 0 {
                    return node.is_end;
                }
            } else {
                return false;
            }
        }

        false
    }

    fn start_with(&self, s: &str) -> Vec<String> {
        let mut res = vec![];
        let mut n = s.len();

        let mut q = vec![];

        let mut root = self;
        for c in s.chars() {
            n -= 1;
            if let Some(node) = root.childs.get(&c) {
                root = node;
                if n <= 0 {
                    q.push((node, s.to_string()));
                }
            }
        }

        // println!("{q:?}");
        // println!("{res:?}");

        while let Some((node, w)) = q.pop() {
            if node.is_end {
                res.push(w.clone());
            }

            for (k, v) in node.childs.iter() {
                let mut nw = w.clone();
                nw.push(*k);

                q.push((v, nw));
            }
        }

        res
    }
}

#[derive(Default, Debug)]
struct Tries {
    root: TrieNode,
}

impl Tries {
    fn new() -> Self {
        Tries::default()
    }

    fn insert(&mut self, s: &str) {
        let mut n = s.len();

        let mut root = &mut self.root;

        for c in s.chars() {
            n -= 1;
            root = root.childs.entry(c).or_insert(TrieNode::empty(n <= 0));
        }
    }

    fn find(&self, q: &str) -> bool {
        self.root.find(q)
    }

    fn start_with(&self, q: &str) -> Vec<String> {
        self.root.start_with(q)
    }
}

fn main() {
    let mut tries = Tries::new();

    tries.insert("a");
    println!("{tries:?}");

    tries.insert("apple");
    println!("{tries:?}");

    tries.insert("apps");
    println!("{tries:?}");

    let res = tries.find("apps");
    println!("Res: {res}");

    let res = tries.find("ap");
    println!("Res: {res}");

    let res = tries.start_with("app");
    println!("RES: {res:?}");
    let res = tries.start_with("a");
    println!("RES: {res:?}");
}
