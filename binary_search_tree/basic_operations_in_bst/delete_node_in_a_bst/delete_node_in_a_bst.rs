/*
 * Author: Nathan Bockisch
 * Date: July 1, 2022
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
    pub fn delete_node(mut root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.delete_key(key);
        root
    }
}

// This trait allows in-place mutation of a tree to delete a node
trait Deletable {
    fn delete_key(&mut self, key: i32);
}

// This trait allows in-place mutation of the successor of a node to insert
// the node's left branch into the successor
trait Successor {
    fn populate_successor(&mut self, prev_left: Rc<RefCell<TreeNode>>);
}

impl Deletable for Option<Rc<RefCell<TreeNode>>> {
    fn delete_key(&mut self, key: i32) {
        if let Some(cur) = self.take() {
            let cur_cmp = key.cmp(&cur.borrow().val);

            match cur_cmp {
                Ordering::Equal => {
                    let left = cur.borrow_mut().left.take();
                    let right = cur.borrow_mut().right.take();

                    match (left, right) {
                        (None, None) => *self = None,
                        (Some(left), None) => *self = Some(left),
                        (None, Some(right)) => *self = Some(right),
                        (Some(left), Some(right)) => {
                            right.borrow_mut().populate_successor(left);
                            *self = Some(right);
                        },
                    }
                },
                Ordering::Greater => {
                    cur.borrow_mut().right.delete_key(key);
                    *self = Some(cur);
                },
                Ordering::Less => {
                    cur.borrow_mut().left.delete_key(key);
                    *self = Some(cur);
                }
            }
        }
    }
}

impl Successor for TreeNode {
    fn populate_successor(&mut self, prev_left: Rc<RefCell<TreeNode>>) {
        let left = self.left.take();

        match left {
            Some(cur_left) => {
                cur_left.borrow_mut().populate_successor(prev_left);
                self.left = Some(cur_left);
            },
            None => self.left = Some(prev_left),
        };
    }
}
