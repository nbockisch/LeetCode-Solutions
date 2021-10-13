/**
 * Author: Nathan Bockisch
 * Date: October 13, 2021
 **/

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public boolean checkSymmetry(TreeNode left, TreeNode right) {
        if (left == null && right == null) {
            return true;
        } else if (left == null || right == null || left.val != right.val) {
            return false;
        } 
        
        return checkSymmetry(left.left, right.right) && 
            checkSymmetry(left.right, right.left);
    }
    
    public boolean isSymmetric(TreeNode root) {
        if (root == null) return true;
        
        return checkSymmetry(root.left, root.right);
    }
}
