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
    public int findMaxDepth(TreeNode cur, int prev_level) {
        if (cur == null) return prev_level;
        
        int level = prev_level + 1;
        int child_max = Math.max(findMaxDepth(cur.left, level), 
                        findMaxDepth(cur.right, level));
        return Math.max(level, child_max);
    }
    
    public int maxDepth(TreeNode root) {
        int level = 0;
        
        if (root != null) 
            level = findMaxDepth(root, level);
        
        return level;
    }
}
