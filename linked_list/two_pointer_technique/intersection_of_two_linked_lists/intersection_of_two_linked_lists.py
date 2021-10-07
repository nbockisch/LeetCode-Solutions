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
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> ListNode:
        a_ptr, b_ptr = headA, headB
        
        # a_ptr will iterate through list a + b
        # b_ptr will iterate through list b + a
        # both hit intersecting list c at the same time
        # if both are None at the same time, there is no
        # intersection and the while loop exits
        while (a_ptr != b_ptr):
            if (not a_ptr):
                a_ptr = headB        
            elif (not b_ptr):
                b_ptr = headA
            else:
                a_ptr = a_ptr.next
                b_ptr = b_ptr.next
        
        return a_ptr
