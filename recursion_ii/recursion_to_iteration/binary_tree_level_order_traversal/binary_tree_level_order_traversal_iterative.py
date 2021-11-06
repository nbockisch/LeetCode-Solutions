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
        if (not root): return []
        
        levels, queue = [], []
        # In the queue for the DFS, node level is saved alongside 
        # node for the return format of this exercise:
        # (an array of arrays with each level's value)
        queue.append((root, 0))
        
        # DFS Traversal
        while (queue):
            tmp = queue.pop(0)
            cur, level = tmp[0], tmp[1]
            
            if (level == len(levels)):
                levels.append([])
                
            levels[level].append(cur.val)
            
            if (cur.left):
                queue.append((cur.left, level + 1))
                
            if (cur.right):
                queue.append((cur.right, level + 1))
        
        return levels
