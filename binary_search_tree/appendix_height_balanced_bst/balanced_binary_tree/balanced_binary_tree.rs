/*
 * Author: Nathan Bockisch
 * Date: July 5, 2022
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
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(cur) = root {
            let l_height = Self::get_height(&cur.borrow().left);
            let r_height = Self::get_height(&cur.borrow().right);
            let height_diff = i32::abs(l_height - r_height);

            if height_diff > 1 { return false }

            let l_balanced = Self::is_balanced(cur.borrow().left.clone());
            let r_balanced = Self::is_balanced(cur.borrow().right.clone());

            return l_balanced && r_balanced;
        }

        true
    }

    fn get_height(cur: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(cur) = cur {
            let l_height = Self::get_height(&cur.borrow().left);
            let r_height = Self::get_height(&cur.borrow().right);

            return 1 + i32::max(l_height, r_height);
        }

        0
    }
}
