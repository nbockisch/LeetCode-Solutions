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
        stack, values = [], []
        cur = root
    
        while (stack or cur):
            if (cur):
                stack.append(cur)
                cur = cur.left
            else:
                cur = stack.pop()
                values.append(cur.val)
                cur = cur.right
            
        return values
