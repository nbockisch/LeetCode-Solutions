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
    public List<Integer> inorderTraversal(TreeNode root) {
        List<Integer> values = new ArrayList<Integer>();
        
        if (root == null) return values;
        
        Stack<TreeNode> node_stack = new Stack<TreeNode>();
        TreeNode cur = root;
        
        while (!node_stack.empty() || cur != null) {
            if (cur != null) {
                node_stack.push(cur);
                cur = cur.left;
            } else {
                cur = node_stack.pop();
                values.add(cur.val);
                cur = cur.right;
            }
        }
        
        return values;
    }
}
