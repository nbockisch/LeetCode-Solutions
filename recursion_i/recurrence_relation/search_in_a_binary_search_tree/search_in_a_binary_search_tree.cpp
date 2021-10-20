/**
 * Author: Nathan Bockisch
 * Date: October 20, 2021
 **/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    TreeNode* searchBST(TreeNode* root, int val) {
        // Base case
        if (!root || root->val == val) return root;
        
        // Check whether we search left or right
        return root->val < val ? searchBST(root->right, val) : searchBST(root->left, val);
    }
};
