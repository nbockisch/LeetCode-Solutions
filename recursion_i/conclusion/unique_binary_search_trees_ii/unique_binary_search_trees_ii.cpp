/**
 * Author: Nathan Bockisch
 * Date: October 26, 2021
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
    map<pair<int, int>, vector<TreeNode *>> cache;
    
    vector<TreeNode *> getTrees(int start, int end) {
        // Check if this tree array already generated
        if (cache.count({start, end})) return cache[{start, end}];
        
        vector<TreeNode *> trees;
        
        // Base case;
        if (start > end) trees.push_back(NULL);
        
        // Get every combination of trees in the given number range
        for (int i = start; i <= end; i++) {
            vector<TreeNode *> left_nodes = getTrees(start, i - 1);
            vector<TreeNode *> right_nodes = getTrees(i + 1, end);
            
            for (TreeNode *left_node : left_nodes) {
                for (TreeNode *right_node : right_nodes) {
                    TreeNode *cur = new TreeNode(i);
                    cur->left = left_node;
                    cur->right = right_node;
                    trees.push_back(cur);
                }
            }
        }
        
        // Save result to avoid duplicate calculations
        cache[{start, end}] = trees;
        
        return trees;
    }
    
    vector<TreeNode*> generateTrees(int n) {
        return getTrees(1, n);
    }
};
