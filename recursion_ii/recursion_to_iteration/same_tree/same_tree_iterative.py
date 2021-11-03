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
            if not p and not q:
                return True
            elif (not p and q) or (p and not q):
                return False
            elif p.val != q.val:
                return False
            
            return True
        
        # Perform a BFS traversal of both and compare each node
        # with its counterpart
        queue = deque()
        queue.append(p)
        queue.append(q)
    
        while (queue):
            a = queue.popleft()
            b = queue.popleft()
         
            if (not isEqual(a, b)):
                return False
            
            if a:
                queue.append(a.left)
                queue.append(b.left)
                queue.append(a.right)
                queue.append(b.right)
        
        return True
