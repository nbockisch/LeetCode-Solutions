'''
Author: Nathan Bockisch
Date: October 11, 2021
'''

"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, next=None):
        self.val = val
        self.next = next
"""

class Solution:
    def insert(self, head: 'Node', insertVal: int) -> 'Node':
        # Case of empty list
        if (not head):
            head = Node(insertVal)
            head.next = head
            return head
        
        prev, cur = head, head.next
        
        while True:
            # Case where value is between min and max of list
            if (prev.val <= insertVal <= cur.val):
                break
            # Case where value is the new min or max of the list
            if ((cur.val < prev.val) and (prev.val <= insertVal or insertVal <= cur.val)):
                break
            
            prev, cur = cur, cur.next
            
            # Case where all values are the same and the head is reached again
            if (prev == head):
                break
        
        prev.next = Node(insertVal, cur)
        return head
