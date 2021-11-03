'''
Author: Nathan Bockisch
Date: November 3, 2021
'''

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        # Helper function to check if two nodes are equal
        def isEqual(p: TreeNode, q: TreeNode) -> bool:
            if (not p and q) or (p and not q):
                return False
            elif p.val != q.val:
                return False
            
            return True
        
        # Do a recursive preorder traversal of both trees and compare
        # each node with its counterpart
        def preorderCheck(p: TreeNode, q: TreeNode) -> bool:
            if (not p and not q):
                return True
            elif (not isEqual(p, q)):
                return False
            
            return preorderCheck(p.left, q.left) and preorderCheck(p.right, q.right)
        
        return preorderCheck(p, q)
