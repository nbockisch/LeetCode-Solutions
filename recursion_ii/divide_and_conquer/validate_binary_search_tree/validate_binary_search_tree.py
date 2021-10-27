'''
Author: Nathan Bockisch
Date: October 27, 2021
'''

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def validateBST(self, cur: Optional[TreeNode], left_bound: int, right_bound: int) -> bool:
        # Base case
        if not cur:
            return True
        
        # If out of bounds, subtree is not valid BST
        if cur.val <= left_bound or cur.val >= right_bound:
            return False
        
        # Check if the subtrees are valid
        return self.validateBST(cur.left, left_bound, cur.val) and self.validateBST(cur.right, cur.val, right_bound)
    
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return self.validateBST(root, -math.inf, math.inf)
