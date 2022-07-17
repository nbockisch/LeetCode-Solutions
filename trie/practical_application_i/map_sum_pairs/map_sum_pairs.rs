/*
 * Author: Nathan Bockisch
 * Date: July 16, 2022
 */
// I used a Trie for this
use std::collections::HashMap;

struct Node {
    sum: i32,
    children: HashMap<char, Node>,
}

impl Node {
    fn new() -> Self {
        Self {
            sum: 0,
            children: HashMap::new(),
        }
    }
}

struct MapSum {
    root: Node,
    values: HashMap<String, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    fn new() -> Self {
        Self {
            root: Node::new(),
            values: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let delta = val - self.values.get(&key).unwrap_or(&0);
        self.values.insert(key.clone(), val);
        let mut cur = &mut self.root;
        cur.sum += delta;

        for c in key.chars() {
            cur = cur.children.entry(c).or_insert(Node::new());
            cur.sum += delta;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut cur = &self.root;

        for c in prefix.chars() {
            match cur.children.get(&c) {
                None => return 0,
                Some(next) => cur = next,
            };
        }

        cur.sum
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
