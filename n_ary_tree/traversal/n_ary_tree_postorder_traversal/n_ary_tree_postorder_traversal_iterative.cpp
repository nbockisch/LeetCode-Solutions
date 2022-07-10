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
    vector<int> postorder(Node* root) {
        vector<int> answer;

        if (root) {
            stack<Node *> node_stack;

            node_stack.push(root);
            while (!node_stack.empty()) {
                Node *cur = node_stack.top();
                node_stack.pop();
                answer.insert(answer.begin(), cur->val);

                for (Node *child : cur->children) {
                    node_stack.push(child);
                }
            }
        }

        return answer;
    }
};
