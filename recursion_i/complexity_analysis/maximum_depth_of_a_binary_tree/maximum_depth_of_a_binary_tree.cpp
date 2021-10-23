/**
 * Author: Nathan Bockisch
 * Date: October 22, 2021
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
    int countMaxDepth(TreeNode* cur, int count) {
        if (!cur) return count;
        
        count++;

        if (!cur->left && !cur->right) return count;
        
        return max(countMaxDepth(cur->left, count), countMaxDepth(cur->right, count));
    }
    
    int maxDepth(TreeNode* root) {
        return countMaxDepth(root, 0);
    }
};
