'''
Author: Nathan Bockisch
Date: October 12, 2021
'''

"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def __init__(self):
        self.known_nodes = {}
    
    def copyRandomList(self, head: 'Node') -> 'Node':
        # Base case
        if (not head):
            return None
        # If already visited, return the clone
        elif (head in self.known_nodes):
            return self.known_nodes[head]
        
        clone = Node(head.val, None, None)
        self.known_nodes[head] = clone
        
        # Recursively clone the next and random nodes
        clone.next = self.copyRandomList(head.next)
        clone.random = self.copyRandomList(head.random)
        
        return clone
