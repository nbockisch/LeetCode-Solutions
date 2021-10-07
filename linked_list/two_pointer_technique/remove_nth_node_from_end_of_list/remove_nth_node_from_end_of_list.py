'''
Author: Nathan Bockisch
Date: October 6, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        slow, fast = head, head
        
        # lookahead pointer gives n distance 
        # between the slow pointer and the end
        for i in range(n):
            fast = fast.next
        
        # if n is the list size, delete the first element
        if (not fast):
            head = head.next
            return head
        
        # get slow to n - 1 places from the end, 
        # then delete the nth element
        while (fast.next):
            fast = fast.next
            slow = slow.next
        
        slow.next = slow.next.next
        return head
