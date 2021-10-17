/**
 * Author: Nathan Bockisch
 * Date: October 17, 2021
 **/

/*
// Definition for a Node.
class Node {
    public int val;
    public Node left;
    public Node right;
    public Node next;

    public Node() {}
    
    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, Node _left, Node _right, Node _next) {
        val = _val;
        left = _left;
        right = _right;
        next = _next;
    }
};
*/

class Solution {
    public Node connect(Node root) {
        if (root == null) return root;
        
        Node cur = root;
        Node next_cur = null; // Pointer to leftmost position on level after cur
        Node child = null; // Pointer to connect nodes on each level
        
        while (cur != null || next_cur != null) {
            while (cur != null) {
                // Set leftmost position of next level
                if (next_cur == null)
                    if (cur.left != null)
                        next_cur = cur.left;
                    else if (cur.right != null)
                        next_cur = cur.right;
                
                // Connect children
                if (cur.left != null) {
                    if (child == null){
                        child = cur.left;
                    } else {
                        child.next = cur.left;
                        child = child.next;
                    }
                }
                
                if (cur.right != null) {
                    if (child == null) {
                        child = cur.right;
                    } else {
                        child.next = cur.right;
                        child = child.next;
                    }
                }     
                
                cur = cur.next;
            }

            /* Move cur to leftmost position on next level
            reset next_cur and child pointers to be set to
            their correct positions on the next level */
            cur = next_cur;
            next_cur = null;
            child = null;
        }
        
        return root;
    }
}
