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

        levelOrderHelper(root, 1, &answer);

        return answer;
    }

    void levelOrderHelper(Node *cur, int level, vector<vector<int>> *answer) {
        if (cur) {
            if (answer->size() < level) {
                vector<int> tmp;
                answer->push_back(tmp);
            }

            answer->at(level - 1).push_back(cur->val);

            for (Node *child : cur->children) {
                levelOrderHelper(child, level + 1, answer);
            }
        }
    }
};
