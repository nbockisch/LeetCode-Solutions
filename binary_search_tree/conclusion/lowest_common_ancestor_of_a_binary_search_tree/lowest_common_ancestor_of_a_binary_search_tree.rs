/*
 * Author: Nathan Bockisch
 * Date: July 3, 2022
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
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let lca_val = Self::lca_helper(&root, p_val, q_val).unwrap();

        Some(Rc::new(RefCell::new(TreeNode::new(lca_val))))
    }

    fn lca_helper(cur: &Option<Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> Option<i32> {
        if let Some(cur) = cur {
            let cur_val = cur.borrow().val;

            return if (p_val > cur_val) && (q_val > cur_val) {
                Self::lca_helper(&cur.borrow().right, p_val, q_val)
            } else if (p_val < cur_val) && (q_val < cur_val) {
                Self::lca_helper(&cur.borrow().left, p_val, q_val)
            } else {
                Some(cur_val)
            }
        }

        None
    }
}
