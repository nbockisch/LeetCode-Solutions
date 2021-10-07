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

1. set prev = head, cur = head.next
Now prev = 1, cur = 2

2. Make prev point to the cur next so we have:
1 -> 3 2 -> 3 -> 4

3. Save cur_next for later use:
Now cur_next = 3

4. Point cur next to head, make head = cur:
2 -> 1 -> 3 -> 4, head = 2

5. Set cur to point to the cur_next saved in 3:
cur = 3

6. Repeat until cur is None, at which point we have:
4 -> 3 -> 2 -> 1, head = 4
'''

class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if (not head):
            return head
        
        prev = head
        cur = head.next
        
        while (cur):
            # unlink prev from cur and link it to cur.next
            # save cur_next for later
            prev.next, cur_next = cur.next, cur.next
            
            # Move cur to the head of the list
            cur.next = head
            head = cur
            
            # Advance to the saved cur next to continue the algorithm
            cur = cur_next
        
        return head
