/*
 * Author: Nathan Bockisch
 * Date: July 16, 2022
 */
struct Node {
    is_word: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new() -> Box<Node> {
        Box::new(Self {
            is_word: false,
            children: Default::default(),
        })
    }
}

struct Trie {
    root: Box<Node>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            root: Node::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            let cur_index = get_index(c);

            if cur.children[cur_index].is_none() {
                cur.children[cur_index] = Some(Node::new());
            }

            cur = cur.children[cur_index].as_mut().unwrap();
        }

        cur.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;

        for c in word.chars() {
            let cur_index = get_index(c);

            match &cur.children[cur_index] {
                None => return false,
                Some(next) => cur = &next,
            };
        }

        cur.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;

        for c in prefix.chars() {
            let cur_index = get_index(c);

            match &cur.children[cur_index] {
                None => return false,
                Some(next) => cur = &next,
            };
        }

        true
    }
}

fn get_index(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
