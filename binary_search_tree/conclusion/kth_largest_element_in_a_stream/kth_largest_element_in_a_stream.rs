/*
 * Author: Nathan Bockisch
 * Date: July 2, 2022
 */

// This particular problem was supposed to be solved using a BST

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub children: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, children: i32) -> Self {
        TreeNode {
            val,
            children,
            left: None,
            right: None
        }
    }
}

trait Insertable {
    fn bst_insert(&mut self, val: i32);
}

impl Insertable for Option<Rc<RefCell<TreeNode>>> {
    // Perform a BST insert on a node
    fn bst_insert(&mut self, val: i32) {
        if let Some(cur) = self {
            if val > cur.borrow().val {
                cur.borrow_mut().right.bst_insert(val);
            } else {
                cur.borrow_mut().left.bst_insert(val);
            }

            cur.borrow_mut().children += 1;
        } else {
            *self = Some(Rc::new(RefCell::new(TreeNode::new(val, 1))));
        }
    }
}

struct KthLargest {
    pub root: Option<Rc<RefCell<TreeNode>>>,
    pub k: i32,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut root = None;

        for val in nums {
            root.bst_insert(val);
        }

        Self { root, k, }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.root.bst_insert(val);
        Self::find_at_k(&self.root, self.k)
    }

    fn find_at_k(cur: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if let Some(cur) = cur {
            // Get the size of right tree, current node is (right_k + 1)'th
            // largest in sequence
            let right_k = match cur.borrow().right {
                Some(ref right) => right.borrow().children,
                None => 0,
            };

            // Check if the current is the kth node
            if (right_k + 1) == k {return cur.borrow().val}

            // Otherwise check if we need to go left or right
            return match k.cmp(&right_k) {
                Ordering::Greater => Self::find_at_k(&cur.borrow().left, k - right_k - 1),
                Ordering::Less | Ordering::Equal => Self::find_at_k(&cur.borrow().right, k),
            };
        }

        0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
