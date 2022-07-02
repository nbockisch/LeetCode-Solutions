/*
 * Author: Nathan Bockisch
 * Date: July 2, 2022
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

trait Insertable {
    fn insert_bst(&mut self, val: i32);
}

impl Insertable for Option<Rc<RefCell<TreeNode>>> {
    fn insert_bst(&mut self, val: i32) {
        if let Some(cur) = self {
            if val > cur.borrow().val {
                cur.borrow_mut().right.insert_bst(val);
            } else {
                cur.borrow_mut().left.insert_bst(val);
            }
        } else {
            *self = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.insert_bst(val);
        root
    }
}
