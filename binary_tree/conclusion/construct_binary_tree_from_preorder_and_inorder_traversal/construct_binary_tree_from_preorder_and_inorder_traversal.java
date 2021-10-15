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
    public TreeNode buildTree(int[] preorder, int[] inorder) {
        // Base case
        switch (inorder.length) {
            case 0:
                return null;
            case 1:
                return new TreeNode(preorder[0]);
        }
        
        // Root of this subtree is always the first of postorder
        TreeNode root = new TreeNode(preorder[0]);
        
        // Get inorder and preorder lists for left and right subtrees
        int split;
        for (split = 0; inorder[split] != root.val; split++);
        
        /* Left and right inorder arrays exclude the root value,
         left and right preorder arrays exclude the first preorder value
         (the current root value) and split at first value of right inorder */
        int in_left[] = Arrays.copyOfRange(inorder, 0, split);
        int pre_left[] = Arrays.copyOfRange(preorder, 1, split + 1);
        int in_right[] = Arrays.copyOfRange(inorder, split + 1, inorder.length);
        int pre_right[] = Arrays.copyOfRange(preorder, split + 1, preorder.length);
        
        // Build the subtree and return it
        root.left = buildTree(pre_left, in_left);
        root.right = buildTree(pre_right, in_right);
        
        return root;
    }
}
