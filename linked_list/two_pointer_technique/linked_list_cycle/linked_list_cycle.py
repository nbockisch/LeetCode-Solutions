'''
Author: Nathan Bockisch
Date: October 6, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
        
    def hasCycle(self, head: ListNode) -> bool:
        slow, fast = head, head
        
        # Fast pointer moves by 2 nodes, slow pointer only by one
        while (fast and fast.next):
            fast = fast.next.next
            slow = slow.next
            
            if fast == slow:
                return True
        
        return False
