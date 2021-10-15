/**
 * Author: Nathan Bockisch
 * Date: October 15, 2021
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
    public TreeNode buildTree(int[] inorder, int[] postorder) {
        // Base case
        switch (inorder.length) {
            case 0:
                return null;
            case 1:
                return new TreeNode(postorder[postorder.length - 1]);
        }
        
        // Root of this subtree is always the last of postorder
        TreeNode root = new TreeNode(postorder[postorder.length - 1]);
        
        // Get inorder and postorder lists for left and right subtrees
        int split;
        for (split = 0; inorder[split] != root.val; split++);
        
        /* Left and right inorder arrays exclude the root value,
         left and right postorder arrays exclude the last postorder value
         (the current root value) and split at first value of right inorder */
        int in_left[] = Arrays.copyOfRange(inorder, 0, split);
        int post_left[] = Arrays.copyOfRange(postorder, 0, split);
        int in_right[] = Arrays.copyOfRange(inorder, split + 1, inorder.length);
        int post_right[] = Arrays.copyOfRange(postorder, split, postorder.length - 1);
        
        // Build the subtree and return it
        root.left = buildTree(in_left, post_left);
        root.right = buildTree(in_right, post_right);
        
        return root;
    }
}
