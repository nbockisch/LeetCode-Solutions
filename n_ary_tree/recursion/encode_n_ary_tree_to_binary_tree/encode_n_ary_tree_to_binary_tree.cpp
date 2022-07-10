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

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

class Codec {
public:
    // Encodes an n-ary tree to a binary tree.
    TreeNode* encode(Node* root) {
        if (!root) return NULL;

        queue<pair<Node *, TreeNode *>> node_queue;
        TreeNode *root_bnode = new TreeNode(root->val);
        pair<Node *, TreeNode *> root_pair = make_pair(root, root_bnode);
        node_queue.push(root_pair);

        while (!node_queue.empty()) {
            pair<Node *, TreeNode *> cur_pair = node_queue.front();
            Node *cur_node = get<0>(cur_pair);
            TreeNode *cur_bnode = get<1>(cur_pair);
            node_queue.pop();

            TreeNode *prev_bnode;
            for (Node *child : cur_node->children) {
                TreeNode *child_bnode = new TreeNode(child->val);
                node_queue.push(make_pair(child, child_bnode));

                if (!prev_bnode) {
                    prev_bnode = child_bnode;
                    cur_bnode->left = child_bnode;
                } else {
                    prev_bnode->right = child_bnode;
                    prev_bnode = child_bnode;
                }
            }
            prev_bnode = NULL;
        }

        return get<1>(root_pair);
        
    }

    void inorderPrint(TreeNode *root) {
        if (!root) return;

        cout << root->val << endl;
        inorderPrint(root->left);
        inorderPrint(root->right);
    }

    // Decodes your binary tree to an n-ary tree.
    Node* decode(TreeNode* broot) {
        if (!broot) return NULL;
        Node *root = new Node(broot->val);

        for (TreeNode* child = broot->left; child; child = child->right) {
            root->children.push_back(decode(child));
        }

        return root;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec codec;
// codec.decode(codec.encode(root));
