use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    childs: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert(&mut self, s: &str) {
        let mut root = self;

        for c in s.chars() {
            root = root.childs.entry(c).or_insert(TrieNode::default());
        }
        root.is_end = true;
    }
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, s: &str) {
        self.root.insert(s);
    }

    fn word_count(&self) -> i32 {
        let mut s: Vec<&TrieNode> = vec![&self.root];
        let mut cnt = 1;

        while let Some(node) = s.pop() {
            if node.is_end {
                cnt += 1;
            }

            for (k, v) in node.childs.iter() {
                s.push(v);
            }
        }

        cnt
    }
}

fn get_sub_strs(s: &str) -> Vec<&str> {
    let mut k = 0;
    let mut i = k;
    let mut res = vec![];

    while k <= i && i < s.len() {
        i += 1;

        res.push(&s[k..i]);

        if i >= s.len() {
            k += 1;
            i = k;
        }
    }

    res
}

fn main() {
    let mut trie = Trie::default();

    let subs = get_sub_strs("abab");
    println!("{subs:?}");

    for sub in subs {
        trie.insert(sub);
    }

    println!("{trie:?}");

    let cnt = trie.word_count();
    println!("Count: {cnt}");

    let mut trie = Trie::default();

    // Test empty string
    // trie.insert("");
    assert_eq!(trie.word_count(), 1);

    // Test single character
    trie.insert("a");
    assert_eq!(trie.word_count(), 2);

    // Test two different characters
    trie.insert("b");
    assert_eq!(trie.word_count(), 3);

    // Test duplicate insertions
    trie.insert("abc");
    trie.insert("abc");
    assert_eq!(trie.word_count(), 4);

    // Test overlapping substrings
    trie.insert("a");
    trie.insert("ab");
    trie.insert("abc");
    assert_eq!(trie.word_count(), 5);

    // Test full distinct substrings
    let mut trie = Trie::default();
    let s = "abc";
    for sub in get_sub_strs(s) {
        trie.insert(&sub);
    }
    assert_eq!(trie.word_count(), 7); // "", "a", "b", "c", "ab", "bc", "abc"

    println!("All tests passed!");
}
