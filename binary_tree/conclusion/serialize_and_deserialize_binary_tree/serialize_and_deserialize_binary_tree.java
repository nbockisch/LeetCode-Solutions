/**
 * Author: Nathan Bockisch
 * Date: October 17, 2021
 **/

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
public class Codec {

    // Encodes a tree to a single string.
    public String serialize(TreeNode root) {
        /* At each node, append the int as a string or 
        null as a string, space delimited */
        
        // Base case
        if (root == null) return "null ";
        
        String tree = String.valueOf(root.val) + " ";
        tree += serialize(root.left);
        tree += serialize(root.right);
        
        return tree;
    }

    // Decodes your encoded data to tree.
    public TreeNode deserialize(String data) {
        /* Split string of node values into array and 
        convert it to an Integer queue */
        String node_str[] = data.split(" ");
        Queue<Integer> nodes = new LinkedList<Integer>();
        
        for (String s : node_str) {
            if (s.equals("null"))
                nodes.add(null);
            else
                nodes.add(Integer.parseInt(s));
        }
                
        return deserializeHelper(nodes);
    }
    
    // Recursively populates the tree with queue of nodes
    public TreeNode deserializeHelper(Queue<Integer> nodes) {
        Integer cur_val = nodes.poll();
        
        // Base case
        if (cur_val == null) return null;
        
        TreeNode cur = new TreeNode(cur_val);
        cur.left = deserializeHelper(nodes);
        cur.right = deserializeHelper(nodes);
        
        return cur;
    }
}

// Your Codec object will be instantiated and called as such:
// Codec ser = new Codec();
// Codec deser = new Codec();
// TreeNode ans = deser.deserialize(ser.serialize(root));
