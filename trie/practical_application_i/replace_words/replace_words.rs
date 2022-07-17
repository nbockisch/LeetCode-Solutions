/*
 * Author: Nathan Bockisch
 * Date: July 17, 2022
 */

// I solved this using a Trie structure
use std::collections::HashMap;

struct Node {
    is_word: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn new() -> Self {
        Self {
            is_word: false,
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(Node::new());
        }

        cur.is_word = true;
    }

    fn search(&self, word: &str) -> Option<usize> {
        let mut root_len = 0;
        let mut cur = &self.root;

        for c in word.chars() {
            match cur.children.get(&c) {
                None => break,
                Some(next) => {
                    cur = next;
                    root_len += 1;

                    if cur.is_word { 
                        return Some(root_len);
                    }
                },
            };
        }

        match cur.is_word {
            true => Some(root_len),
            false => None,
        }
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(&word);
        }

        let mut answer = String::new();

        for word in sentence.split_whitespace() {
            let root_len = match trie.search(&word) {
                None => word.len(),
                Some(len) => len,
            };

            answer.push_str(&word[0..root_len]);
            answer.push(' ');
        }

        answer.pop();
        answer
    }
}
