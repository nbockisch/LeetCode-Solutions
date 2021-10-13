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
    public boolean isSymmetric(TreeNode root) {
        if (root == null) return true;
        
        Queue<TreeNode> visited = new LinkedList<TreeNode>();
        TreeNode ptr_left = root.left;
        TreeNode ptr_right = root.right;
        visited.add(ptr_right);
        visited.add(ptr_left);
        
        while (!visited.isEmpty()) {
            ptr_left = visited.poll();
            ptr_right = visited.poll();
            
            if (ptr_left == null && ptr_right == null) {
                continue;
            } else if (ptr_left == null || ptr_right == null || 
                       ptr_left.val != ptr_right.val) {
                return false;
            }
            
            visited.add(ptr_left.left);
            visited.add(ptr_right.right);
            visited.add(ptr_left.right);
            visited.add(ptr_right.left);
        }
        
        return true;
    }
}
