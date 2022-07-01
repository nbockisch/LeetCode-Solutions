/*
 * Author: Nathan Bockisch
 * Date: June 26, 2022
 */

use std::rc::Rc;
use std::cell::RefCell;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bst_iter = Self { stack: Vec::new() };
        bst_iter.populate_left(root);
        bst_iter
    }

    // Return the value at the top of the stack, and check if right node exists
    // that needs left children pushed to the stack along with itself
    fn next(&mut self) -> i32 {
        let stack_item = self.stack.pop().unwrap();
        let mut node = stack_item.borrow_mut();
        self.populate_left(node.right.take());
        node.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    // Push all left leaves of the tree to the stack, giving O(h) memory
    fn populate_left(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let left = node.borrow_mut().left.take();
            self.stack.push(node);
            self.populate_left(left);
        }
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
