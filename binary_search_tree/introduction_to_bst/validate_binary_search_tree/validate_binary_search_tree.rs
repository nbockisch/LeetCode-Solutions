/*
 * Author: Nathan Bockisch
 * Date: January 7, 2022
 */

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
use std::i64;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_bst(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(cur) = node {
                let cur_val = cur.borrow().val as i64;
                
                if cur_val <= min || cur_val >= max {
                    return false;
                }
                
                return check_bst(&cur.borrow().left, min, cur_val) 
                && check_bst(&cur.borrow().right, cur_val, max);
            }
            
            return true;
        }
        
        // Use i64 to avoid out of bounds problems with i32
        return check_bst(&root, i64::MIN, i64::MAX);
    }
}
