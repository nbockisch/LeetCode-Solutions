'''
Author: Nathan Bockisch
Date: October 7, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        start = ListNode(None)
        start.next = head
        
        # Have two pointers, one behind head and one at head
        fast, slow = head, start
        
        # When fast pointer finds a non-val value, make it the next
        # element in the list, skipping all val elements
        while (fast):
            if (fast.val == val):
                fast = fast.next
                continue
            slow.next = fast
            slow = slow.next
            fast = fast.next
            
        # If all elements are val, make sure the list head is null
        slow.next = fast
        return start.next
