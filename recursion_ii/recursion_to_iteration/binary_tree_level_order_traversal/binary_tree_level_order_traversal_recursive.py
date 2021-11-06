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
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        levels = []
        
        def traverse(cur, level):
            if (not cur): return
            
            # Start a level's array if it doesn't exist yet
            if (level == len(levels)):
                levels.append([])
            
            levels[level].append(cur.val)
            
            # Get the values on the next level
            if (cur.left):
                traverse(cur.left, level + 1)
            
            if (cur.right):
                traverse(cur.right, level + 1)
                
        traverse(root, 0)
        return levels
