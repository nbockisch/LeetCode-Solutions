'''
Author: Nathan Bockisch
Date: November 5, 2021
'''
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:   
        def traverse(cur: TreeNode, nodes: list) -> List[int]:
            if (cur):
                traverse(cur.left, nodes)
                nodes.append(cur.val)
                traverse(cur.right, nodes)
            
            return nodes
        
        return traverse(root, [])
