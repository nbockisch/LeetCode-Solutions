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
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> levels = new ArrayList<List<Integer>>();
        Queue<TreeNode> visited = new LinkedList<TreeNode>();
        
        if (root == null) return levels;
        visited.add(root);
        TreeNode cur;
        int level = 0;
        
        // When queue is empty, all nodes have been visited
        while (!visited.isEmpty()) {
            // Add a new array for the level for the desired output format
            levels.add(new ArrayList<Integer>());
            int cur_size = visited.size();
            
            /* Add all the nodes from the last level to the right
            list in the output, while queueing up the next level's
            elements */
            for (int i = 0; i < cur_size; i++) {
                cur = visited.poll();
                levels.get(level).add(cur.val);
            
                if (cur.left != null) visited.add(cur.left);
                if (cur.right != null) visited.add(cur.right);      
            }

            level++;
        }
        
        return levels;
    }
}
