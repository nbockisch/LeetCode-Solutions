'''
Author: Nathan Bockisch
Date: October 9, 2021
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # The merged list (starts on l3.next)
        l3 = ListNode()
        
        l1_ptr = l1
        l2_ptr = l2
        l3_ptr = l3
        
        # Merge the lists into l3
        while (l1_ptr and l2_ptr):
            if (l1_ptr.val <= l2_ptr.val):
                l3_ptr.next = l1_ptr
                l1_ptr = l1_ptr.next
            else:
                l3_ptr.next = l2_ptr
                l2_ptr = l2_ptr.next
                
            l3_ptr = l3_ptr.next
        
        # Add any leftover elements from l1 or l2
        while (l1_ptr):
            l3_ptr.next = l1_ptr
            
            l1_ptr = l1_ptr.next
            l3_ptr = l3_ptr.next
        
        while (l2_ptr):
            l3_ptr.next = l2_ptr
            
            l2_ptr = l2_ptr.next
            l3_ptr = l3_ptr.next
            
        return l3.next
