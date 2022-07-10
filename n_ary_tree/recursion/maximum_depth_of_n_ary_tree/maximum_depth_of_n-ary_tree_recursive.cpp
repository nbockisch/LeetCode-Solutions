/*
 * Author: Nathan Bockisch
 * Date: July 10, 2022
 */
/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    int maxDepth(Node* root) {
        if (!root) return 0;

        int cur_max = 0;
        for (Node *child : root->children) {
            cur_max = max(cur_max, maxDepth(child));
        }

        return 1 + cur_max;
    }
};
