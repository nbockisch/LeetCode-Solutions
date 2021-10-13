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
    public List<Integer> preorderTraversal(TreeNode root) {
        List<Integer> values = new ArrayList<Integer>();
        if (root == null) return values;
        
        Stack<TreeNode> node_stack = new Stack<TreeNode>();
        TreeNode cur;
        node_stack.push(root);
        
        /* Pop and add the root node value, then put left 
        as next root node on top of the stack, followed by right */
        while (!node_stack.empty()) {
            cur = node_stack.pop();
            values.add(cur.val);
            
            if (cur.right != null) node_stack.push(cur.right);
            if (cur.left != null) node_stack.push(cur.left);
        }
        
        return values;
    }
}
