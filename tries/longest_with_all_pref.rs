use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    childs: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert(&mut self, s: &str) {
        let mut root = self;
        let mut n = s.len();
        for c in s.chars() {
            n -= 1;
            let mut node = TrieNode::default();
            if n <= 0 {
                node.is_end = true;
            }
            root = root
                .childs
                .entry(c)
                .and_modify(|v| {
                    if n <= 0 {
                        v.is_end = true;
                    }
                })
                .or_insert(node);
        }
    }

    fn longest_with_ps(&self) -> String {
        let mut s = vec![(self, String::new())];

        for (k, v) in self.childs.iter() {
            s.push((v, String::from(*k)));
        }

        let mut maxi = String::new();

        while let Some((node, w)) = s.pop() {
            if !node.is_end {
                continue;
            }

            if maxi.len() < w.len() || (maxi.len() == w.len() && maxi > w) {
                maxi = w.clone();
            }

            for (k, v) in node.childs.iter() {
                let mut w = w.clone();
                w.push(*k);
                s.push((v, w));
            }
        }

        maxi
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

    fn longest_with_ps(&self) -> String {
        self.root.longest_with_ps()
    }
}

fn dump(trie: &Trie) {
    let mut s = vec![(&trie.root, String::new())];
    let mut res = vec![];

    while let Some((node, w)) = s.pop() {
        if node.is_end {
            res.push(w.clone());
        }
        for (k, v) in node.childs.iter() {
            let mut w = w.clone();
            w.push(*k);
            s.push((v, w));
            // w.push(*k);
            // s.push((v, w.clone()));
        }
    }

    println!("{res:?}");
}

fn main() {
    let mut trie = Trie::default();
    trie.insert("app");
    trie.insert("apple");
    trie.insert("apples");

    trie.insert("n");
    trie.insert("ni");
    trie.insert("nin");
    trie.insert("ninj");
    trie.insert("ninja");
    trie.insert("ninga");
    trie.insert("ninje");

    // println!("Res: {trie:?}");

    // dump(&trie);
    let res = trie.root.longest_with_ps();
    println!("Res: {res:?}");

    let mut trie = Trie::default();
    trie.insert("ab");
    trie.insert("bc");
    trie.insert("b");
    let res = trie.longest_with_ps();
    println!("Res: {res:?}");

    let mut trie = Trie::new();

    // Test Case 1: Basic Case
    let words1 = vec!["a", "ap", "app", "appl", "apple", "apply"];
    for &word in &words1 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "apple"); // ✅ All prefixes exist.

    // Test Case 2: Multiple Valid Words
    let mut trie = Trie::new();
    let words2 = vec!["t", "te", "tes", "test", "teste", "tester"];
    for &word in &words2 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "tester"); // ✅ All prefixes exist.

    // Test Case 3: Words Without Complete Prefixes
    let mut trie = Trie::new();
    let words3 = vec!["h", "he", "hel", "hello", "world", "wor", "word"];
    for &word in &words3 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "hel"); // ✅ "hello" is invalid (missing "hell").

    // Test Case 4: Only Single Letters
    let mut trie = Trie::new();
    let words4 = vec!["a", "b", "c", "d"];
    for &word in &words4 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "a"); // ✅ Any single letter is valid.

    // Test Case 5: Fixed (Correct Answer: "bcd")
    let mut trie = Trie::new();
    let words5 = vec!["ab", "abc", "abcd", "b", "bc", "bcd"];
    for &word in &words5 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "bcd"); // ✅ "bcd" is longest & valid.

    // Test Case 6: Large Input
    let mut trie = Trie::new();
    let words6 = vec![
        "a",
        "ab",
        "abc",
        "abcd",
        "abcde",
        "abcdef",
        "abcdefg",
        "abcdefgh",
        "abcdefghi",
        "abcdefghij",
        "abcdefghijk",
    ];
    for &word in &words6 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "abcdefghijk"); // ✅ All prefixes exist.

    // Test Case 7: Similar Length Words
    let mut trie = Trie::new();
    let words7 = vec![
        "c",
        "ca",
        "cat",
        "cate",
        "cater",
        "caterp",
        "caterpi",
        "caterpil",
        "caterpill",
        "caterpilla",
        "caterpillar",
    ];
    for &word in &words7 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "caterpillar"); // ✅ All prefixes exist.

    // Test Case 8: Lexicographically Smallest for Tiebreaker
    let mut trie = Trie::new();
    let words8 = vec!["a", "ab", "ac", "abc", "acd"];
    for &word in &words8 {
        trie.insert(word);
    }
    assert_eq!(trie.longest_with_ps(), "abc"); // ✅ Same length as "acd" but smaller.

    println!("All test cases passed!");
}
