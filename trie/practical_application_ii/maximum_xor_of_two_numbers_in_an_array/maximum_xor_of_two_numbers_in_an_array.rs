/*
 * Author: Nathan Bockisch
 * Date: July 22, 2022
 */
use std::cmp::max;

#[derive (Default)]
struct Node {
    children: [Option<Box<Node>>; 2],
}

impl Node {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, num: &i32) {
        let mut cur = self;

        for i in (0..31).rev() {
            let bit_val = ((num >> i) & 1) as usize;

            if cur.children[bit_val].is_none() {
                //println!("bit: {}", bit_val);
                cur.children[bit_val] = Some(Box::new(Self::new()));
            }

            cur = cur.children[bit_val].as_mut().unwrap();
        }
    }

    fn get_xor_val(&self, num: &i32) -> i32 {
        let mut cur = self;
        let mut cur_val = 0;

        for i in (0..31).rev() {
            let cur_bit = ((num >> i) & 1) as usize;

            match &cur.children[1 - cur_bit] {
                Some(next) => {
                    cur_val |= 1 << i;
                    cur = next;
                },
                None => cur = &cur.children[cur_bit].as_ref().unwrap(),
            };
        }

        cur_val
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut root = Node::new();

        // Populate tree
        for num in &nums { root.insert(num); }

        let mut max_val = 0;
        for num in &nums {
            max_val = max(max_val, root.get_xor_val(num));
        }

        max_val
    }
}
