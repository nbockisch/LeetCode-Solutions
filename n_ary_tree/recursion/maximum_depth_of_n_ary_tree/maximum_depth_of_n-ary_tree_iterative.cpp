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
    // Use a breadth-first search to get the max depth
    int maxDepth(Node* root) {
        int depth = 0;

        if (root) {
            queue<Node *> node_queue;
            node_queue.push(root);

            while (!node_queue.empty()) {
                depth++;
                int queue_size = node_queue.size();

                for (int i = 0; i < queue_size; i++) {
                    Node *cur = node_queue.front();
                    node_queue.pop();

                    for (Node *child : cur->children) {
                        node_queue.push(child);
                    }
                }
            }
        }

        return depth;
    }
};
