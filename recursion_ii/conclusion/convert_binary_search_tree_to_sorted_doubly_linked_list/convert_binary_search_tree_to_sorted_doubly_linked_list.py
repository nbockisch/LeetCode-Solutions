'''
Author: Nathan Bockisch
Date: November 7, 2021
'''

"""
# Definition for a Node.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
"""

class Solution:
    def treeToDoublyList(self, root: 'Node') -> 'Node':
        if (not root): return root
        
        # Link the nodes with inorder traversal
        def linkTree(cur: 'Node') -> None:
            if (not cur): return
            
            linkTree(cur.left)
            
            nonlocal head, prev
            
            # Set the head to the minimum (furthest left) value
            if (not head): head = cur
            
            # Doubly link the last seen node with the current
            if (prev):
                prev.right = cur
                cur.left = prev
            
            # Set up the next node to link with the current
            prev = cur
            
            linkTree(cur.right)
        
        head, prev = None, None
        
        linkTree(root)
        
        # Link the first and last nodes to make the list circular
        prev.right = head
        head.left = prev
        
        return head
