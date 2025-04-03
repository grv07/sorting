use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_end: bool,
    childs: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn insert(&mut self, s: &str) {
        let mut root = self;
        for c in s.chars() {
            root = root.childs.entry(c).or_insert(TrieNode::default());
        }

        root.is_end = true;
    }

    fn max_xor(&self, q: &str) -> i32 {
        let mut res = String::new();
        let mut root = self;
        for q in q.chars() {
            root = if q == '0' && root.childs.contains_key(&'1') {
                res.push('1');
                root.childs.get(&'1').unwrap()
            } else if q == '1' && root.childs.contains_key(&'0') {
                res.push('0');
                root.childs.get(&'0').unwrap()
            } else {
                res.push(q);
                root.childs.get(&q).unwrap()
            };
        }

        i32::from_str_radix(&res, 2).unwrap()
    }
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, s: &str) {
        self.root.insert(s);
    }

    fn max_xor(&self, q: &str) -> i32 {
        self.root.max_xor(q)
    }
}

fn main() {
    let a = &[6, 8, 5, 17];
    let b = &[7, 8, 2, 14];

    let mut trie = Trie::new();

    for i in b {
        let s = format!("{:032b}", i);

        // println!("{:?}", &i);
        // println!("{:?}", &s);
        trie.insert(&s);
    }

    // println!("{trie:?}");

    let mut maxi = 0;
    for i in a {
        let s = format!("{:032b}", i);

        // println!("{:?}", &s);

        let res = trie.max_xor(&s);
        // println!("I: {i} Res: {res}, XOR: {}", res ^ i);
        maxi = maxi.max(res ^ i);
    }
    println!("{:?}", maxi);
}
