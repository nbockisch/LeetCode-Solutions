'''
Author: Nathan Bockisch
Date: October 9, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

'''
Algorithm explanation:
Given: 1 -> 2 -> 3 -> 4 -> 5
Output: 1 -> 3 -> 5 -> 2 -> 4
Set odd = head = 1, even = head.next = 2, and even_h = even = 2

1. Set odd.next = even.next:
1 -> 3 -> 4 ..., 2 -> 3 -> 4 ...

Set odd = odd.next
odd = 3

Set even.next = odd.next
1 -> 3 -> 4 ..., 2 -> 4 -> 5

Set even = even.next:
even = 4

2. Break step 1 loop when not head or not head.next, then you have:
odd = 5, even = 4
1 -> 3 -> 5, even_h = 2, 2 -> 4

Set odd.next = even_h:
1 -> 3 -> 5 -> 2 -> 4
'''
class Solution:
        
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # If list size is 0 or 1, algorithm doesn't apply
        if (not head or not head.next):
            return head
        
        odd = head
        even = head.next
        # head of even list
        even_h = even
        
        # Put next odd element at end of list and
        # add even element to even_h list
        while (even and even.next):
            odd.next = even.next
            odd = odd.next
            even.next = odd.next
            even = even.next
        
        # Join the odd and even lists to get the full list
        odd.next = even_h
        return head
        
