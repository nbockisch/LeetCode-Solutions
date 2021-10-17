/**
 * Author: Nathan Bockisch
 * Date: October 17, 2021
 **/

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        // Base case, we either found p, q, or null
        if (root == null || root.val == p.val || root.val == q.val) return root;
        
        TreeNode left = lowestCommonAncestor(root.left, p, q);
        TreeNode right = lowestCommonAncestor(root.right, p, q);
        
        // If both children are null, value doesn't exist in this subtree
        if (left == null && right == null)
            return null;
        // If both children not null, both values found in this subtree
        else if (left != null && right != null)
            return root;
        // If left is not null, left subtree has one or both values
        else if (left != null)
            return left;
        // If right is not null, right subtree has one or both values
        else
            return right;
        
    }
}
