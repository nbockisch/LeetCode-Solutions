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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) 
    -> Option<Rc<RefCell<TreeNode>>> {

        match root.clone() {
            None => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None
            }))),
            Some(cur) => {
                let mut cur = cur.borrow_mut();

                if (val > cur.val) {
                    cur.right = Self::insert_into_bst(cur.right.clone(), val);
                } else {
                    cur.left = Self::insert_into_bst(cur.left.clone(), val);
                }

                root
            },
        }
    }
}
