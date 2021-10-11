'''
Author: Nathan Bockisch
Date: October 11, 2021
'''

"""
# Definition for a Node.
class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child
"""

class Solution:
    def flatten(self, head: 'Node') -> 'Node':
        stack = deque()
        
        cur = head
        while (cur or stack):
            # If child found, connect it as next of cur,
            # and save the previous cur.next value if it exists
            if (cur.child):
                if (cur.next):
                    stack.append(cur.next)
                cur.next = cur.child
                cur.child.prev = cur 
            # If at end of list, check if previous cur.next
            # value is saved and if so attach it to the end of
            # the list
            elif (not cur.next and stack):
                cur.next = stack.pop()
                cur.next.prev = cur
                
            cur.child = None
            cur = cur.next
        
        return head
