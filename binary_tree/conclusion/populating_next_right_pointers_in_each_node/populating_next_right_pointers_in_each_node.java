/**
 * Author: Nathan Bockisch
 * Date: October 16, 2021
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
        Node next_cur = root.left;
        
        // If next cur position is null, we're on the last level
        while (next_cur != null) {
            /* Connect left to right, then right to next left,
            and keep moving cur to the next node in the level */
            while (cur != null) {
                cur.left.next = cur.right;
                
                if (cur.next != null) {
                    cur.right.next = cur.next.left;
                }
                
                cur = cur.next;
            }
            
            /* Move cur to the saved leftmost position on the next level,
            and save the leftmost position on the level after that */
            cur = next_cur;
            next_cur = cur.left;
        }
        
        return root;
    }
}
