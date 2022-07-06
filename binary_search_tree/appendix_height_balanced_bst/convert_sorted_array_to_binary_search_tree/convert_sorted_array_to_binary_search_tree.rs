/*
 * Author: Nathan Bockisch
 * Date: July 6, 2022
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_balanced_bst(&nums, 0, nums.len())
    }

    fn build_balanced_bst(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        match start < end {
            true => {
                let mid = (start + end - 1) / 2;

                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: Self::build_balanced_bst(nums, start, mid),
                    right: Self::build_balanced_bst(nums, mid + 1, end),
                })))
            },
            false => None,
        }
    }
}
