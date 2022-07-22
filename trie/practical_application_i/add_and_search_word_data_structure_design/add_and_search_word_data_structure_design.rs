/*
 * Author: Nathan Bockisch
 * Date: July 21, 2022
 */
// A Trie node
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_word: false,
        }
    }
}

struct WordDictionary {
    root: Node,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn add_word(&mut self, word: String) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            let cur_index = Self::get_index(c);

            if let None = cur.children[cur_index] {
                cur.children[cur_index] = Some(Box::new(Node::new()));
            }

            cur = cur.children[cur_index].as_mut().unwrap();
        }

        cur.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        Self::search_helper(&self.root, &word.chars().collect::<Vec<_>>()[..])
    }

    fn search_helper(cur: &Node, word: &[char]) -> bool {
        if let Some(&c) = word.first() {
            if c == '.' {
                for child in &cur.children {
                    if let Some(next) = child {
                        if Self::search_helper(&next, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else if let Some(next) = &cur.children[Self::get_index(c)] {
                return Self::search_helper(&next, &word[1..]);
            }

            return false;
        }

        cur.is_word
    }

    // Convert a char to a usize index
    fn get_index(c: char) -> usize {
        (c as u8 - 'a' as u8) as usize
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
