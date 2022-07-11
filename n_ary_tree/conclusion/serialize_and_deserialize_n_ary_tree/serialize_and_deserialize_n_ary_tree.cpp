/*
 * Author: Nathan Bockisch
 * Date: July 11, 2022
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

class Codec {
public:
    // Encodes a tree to a single string.
    // Here, I chose to encode it in the LeetCode tree serialization format
    string serialize(Node* root) {
        if (!root) return "";

        string serial = "";
        queue<Node *> node_queue;
        node_queue.push(root);
        serial += (to_string(root->val) + ",null");

        while (!node_queue.empty()) {
            Node *cur = node_queue.front();
            node_queue.pop();

            for (Node *child : cur->children) {
                serial += ("," + to_string(child->val));
                node_queue.push(child);
            }

            serial += ",null";
        }

        return serial;
    }
	
    // Decodes your encoded data to tree.
    Node* deserialize(string data) {
        vector<string> tokens;

        stringstream ss(data);
        string buf;
        while (getline(ss, buf, ',')) {
            tokens.push_back(buf);
        }

        queue<Node *> node_queue;
        Node *root = NULL;
        Node *cur = NULL;
        for (string tok : tokens) {
            if (tok == "null") {
                cur = node_queue.front();
                node_queue.pop();
                continue;
            };

            Node *child = new Node(stoi(tok));
            node_queue.push(child);
            if (!root) root = child;
            if (cur) cur->children.push_back(child);
        }

        return root;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec codec;
// codec.deserialize(codec.serialize(root));
