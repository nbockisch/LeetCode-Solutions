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
    public List<Integer> postorderTraversal(TreeNode root) {
        List<Integer> values = new ArrayList<Integer>();
        
        if (root == null) return values;
        
        Stack<TreeNode> node_stack = new Stack<TreeNode>();
        node_stack.push(root);
        TreeNode cur;
        
        /* Add the nodes at the end of the tree to the list,
        non-end nodes get their left nodes pushed on the stack
        on top of their right nodes, so they get popped in order */
        while (!node_stack.empty()) {
            cur = node_stack.peek();
            if (cur.left == null && cur.right == null) {
                values.add(node_stack.pop().val);
            } else  {
                if (cur.right != null) {
                    node_stack.add(cur.right);
                    cur.right = null;
                } 
                if (cur.left != null) {
                    node_stack.add(cur.left);
                    cur.left = null;
                }
            }
        }
        
        return values;
    }
}
