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
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, 
        p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

        // Do an inorder traversal, setting the successor at the first node greater than p
        fn get_successor(cur: &Option<Rc<RefCell<TreeNode>>>, 
            p_val: i32, 
            successor: &mut Option<Rc<RefCell<TreeNode>>>) {

            if let Some(cur) = cur {
                let cur = cur.borrow();

                get_successor(&cur.left, p_val, successor);
                if (successor.is_none()) && (cur.val > p_val) {
                    *successor = Some(Rc::new(RefCell::new(
                        TreeNode{ val: cur.val, left: None, right: None}
                    )));
                }
                get_successor(&cur.right, p_val, successor);
            }
        }

        let mut successor = None;

        if let Some(val) = p {
            get_successor(&root, val.borrow().val, &mut successor);
        }

        successor
    }
}
