/**
 * Author: Nathan Bockisch
 * Date: October 14, 2021
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
    public int countUnivalSubtrees(TreeNode root) {
        if (root == null) return 0;
        
        return checkUnival(root)[0];
    }
    
    public int[] checkUnival(TreeNode cur) {
        // Index 0 is the current count of unival subtrees
        // Index 1 is whether this subtree is unival (1) or not (0)
        int[] ret_vals = new int[2];
        
        // If leaf node, subtree is unival
        if (cur.left == null && cur.right == null) {
            ret_vals[0]++;
            ret_vals[1] = 1;
        } else if (cur.left == null) { // Only right child exists
            int[] right_vals = checkUnival(cur.right);
            ret_vals[0] += right_vals[0];
            
            /* If cur same as children, and all children are unival,
            cur with parent is also unival */
            if (cur.val == cur.right.val && right_vals[1] == 1) {
                ret_vals[0]++;
                ret_vals[1] = right_vals[1];
            }
        } else if (cur.right == null) { // Only left child exists
            int[] left_vals = checkUnival(cur.left);
            ret_vals[0] += left_vals[0];
            
            /* If cur same as children, and all children are unival,
            cur with parent is also unival */
            if (cur.val == cur.left.val && left_vals[1] == 1) {
                ret_vals[0]++;
                ret_vals[1] = left_vals[1];
            }
        } else { // Both children exist
            int[] left_vals = checkUnival(cur.left);
            int[] right_vals = checkUnival(cur.right);
            
            ret_vals[0] += (left_vals[0] + right_vals[0]);
            
            /* If cur same as children, and all children are unival,
            cur with parent is also unival */
            if (cur.left.val == cur.right.val &&
               cur.left.val == cur.val &&
               left_vals[1] == 1 && right_vals[1] == 1) {
                ret_vals[0]++;
                ret_vals[1] = left_vals[1];
            }
        }
        
        return ret_vals;
    }
}
