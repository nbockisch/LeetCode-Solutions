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
    def detectCycle(self, head: ListNode) -> ListNode:
        fast, slow = head, head
        has_cycle = False
        
        # Find if cycle exists
        while (fast and fast.next):
            fast = fast.next.next
            slow = slow.next
            
            if (fast == slow):
                has_cycle = True
                break
        
        # If a cycle exists, find where fast wraps 
        # around and meets slow
        if has_cycle:
            slow = head
            while (slow != fast):
                slow = slow.next
                fast = fast.next
            
            return slow
        
        return None
