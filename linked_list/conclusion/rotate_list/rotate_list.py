'''
Author: Nathan Bockisch
Date: October 12, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        # Deal with empty list, or no rotation
        if (not head or k <= 0):
            return head
        
        # Make list circular
        cur = head
        len = 1
        while (cur and cur.next):
            cur = cur.next
            len += 1
        cur.next = head
        
        # Head is at len - k
        # to account for k > len, mod it by len - 1,
        # the -1 is for 0 indexing
        
        # Break list at new tail and set new head right after
        cur = head
        for i in range(len - k % len - 1):
            cur = cur.next
        
        head = cur.next
        cur.next = None
        
        return head
