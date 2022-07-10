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
    vector<vector<int>> levelOrder(Node* root) {
        vector<vector<int>> answer;

        if (root) {
            queue<Node *> node_queue;
            node_queue.push(root);

            while (!node_queue.empty()) {
                int level_len = node_queue.size();
                vector<int> cur_vals;

                for (int i = 0; i < level_len; i++) {
                    Node *cur = node_queue.front();
                    node_queue.pop();
                    cur_vals.push_back(cur->val);

                    for (Node *child : cur->children) {
                        node_queue.push(child);
                    }
                }

                answer.push_back(cur_vals);
            }
        }

        return answer;
    }
};
