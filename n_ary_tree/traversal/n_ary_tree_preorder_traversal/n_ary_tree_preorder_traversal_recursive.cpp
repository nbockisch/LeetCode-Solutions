/*
 * Author: Nathan Bockisch
 * Date: July 9, 2022
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
    vector<int> preorder(Node* root) {  
        vector<int> answer;

        preorder_helper(root, &answer);

        return answer;
    }
    
    void preorder_helper(Node *cur, vector<int> *answer) {
        if (cur) {
            answer->push_back(cur->val);

            for (Node *child : cur->children) {
                preorder_helper(child, answer);
            }
        }
    }
};
