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
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(cur) => {
                let cur_cmp = val.cmp(&cur.borrow().val);

                // Check if the value is found, or if we need to go left or right
                match cur_cmp {
                    Ordering::Equal => Some(cur),
                    Ordering::Greater => {
                        let mut cur = cur.borrow_mut();
                        Self::search_bst(cur.right.take(), val)
                    },
                    Ordering::Less => {
                        let mut cur = cur.borrow_mut();
                        Self::search_bst(cur.left.take(), val)
                    }
                }
            }
        }
    }
}
