'''
Author: Nathan Bockisch
Date: October 7, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

'''
Algorithm explanation:
Say you start with:
1 -> 2 -> 3 -> 4

1. Recursively call the function on each element until you reach the end:
head = 4, head.next = null
return head

2. Set cur = last function call cur (the head returned from the end of the list)
head = 3, cur = 4

3. Make the element after head point to head, and erase head's connections, 
making it the end of the list:
1 -> 2 -> 3 -> None, 4 -> 3

In the next step this becomes:
1 -> 2 -> None, 4 -> 3 -> 2

And so on

4. At the end of the function call stack, at the first function call, return cur,
which will be the head set at the last function call:
cur = 4, 4 -> 3 -> 2 -> 1
'''

class Solution:
    
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # Base case
        if (not head or not head.next):
            return head
        
        cur = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        
        return cur
