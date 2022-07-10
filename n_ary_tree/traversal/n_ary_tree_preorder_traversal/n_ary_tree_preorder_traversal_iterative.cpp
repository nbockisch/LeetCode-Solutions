/*
 * Author: Nathan Bockisch
 * Date: July 9, 2022
 */
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

        if (root) {
            std::stack<Node*> node_stack;
            node_stack.push(root);

            while (!node_stack.empty()) {
                Node *cur = node_stack.top();
                node_stack.pop();
                answer.push_back(cur->val);

                for (auto child = cur->children.rbegin(); child != cur->children.rend(); child++) {
                    node_stack.push(*child);
                }
            }
        }

        return answer;
    }
};
