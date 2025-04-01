use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    childs: HashMap<char, TrieNode>,
    wc: i32,
    pc: i32,
}

impl TrieNode {
    fn new(wc: i32, pc: i32) -> Self {
        TrieNode {
            wc,
            pc,
            ..Default::default()
        }
    }

    fn insert(&mut self, c: char, _is_first: bool, is_end: bool) -> &mut TrieNode {
        self.childs
            .entry(c)
            .and_modify(|v| {
                v.pc += 1;
                if is_end {
                    v.wc += 1;
                }
            })
            .or_insert_with(|| {
                let pc = 1;
                let wc = if is_end { 1 } else { 0 };
                TrieNode::new(wc, pc)
            })
    }

    fn count_word(&self, s: &str) -> i32 {
        let mut root = self;
        // let mut n = s.len();
        for c in s.chars() {
            if let Some(node) = root.childs.get(&c) {
                // n -= 1;
                root = node;
            } else {
                return 0;
            }
        }

        root.wc
    }

    fn count_words_starting_with(&self, s: &str) -> i32 {
        let mut root = self;
        for c in s.chars() {
            if let Some(node) = root.childs.get(&c) {
                root = node;
            } else {
                break;
            }
        }

        root.pc
    }
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, s: &str) {
        let mut n = s.len();
        let mut first = true;
        let mut root = &mut self.root;
        for c in s.chars() {
            n -= 1;
            root = root.insert(c, first, n <= 0);
            first = false;
        }
    }

    fn count_word(&self, s: &str) -> i32 {
        self.root.count_word(s)
    }

    fn count_words_starting_with(&self, s: &str) -> i32 {
        self.root.count_words_starting_with(s)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple");
    trie.insert("apps");
    trie.insert("app");
    trie.insert("app");
    trie.insert("app");
    println!("Res: {trie:?}");

    let res = trie.count_word("apps");
    println!("Res: {res:?}");

    let res = trie.count_word("app");
    println!("Res: {res:?}");

    let res = trie.count_words_starting_with("ap");
    println!("Res: {res:?}");

    let res = trie.count_words_starting_with("a");
    println!("Res: {res:?}");
}
