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
    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) return false;
        // Every call, subtract the target val by the current val
        // a path is found if targetSum is 0 at the leaf node
        targetSum -= root.val;
        
        // If a leaf node, targetSum should now be 0 if a path was found
        if (root.left == null && root.right == null) {
            return targetSum == 0;
        }
        
        return hasPathSum(root.left, targetSum) || hasPathSum(root.right, targetSum);
    }
}
