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
    # Helper function to get the ones place digit of a number
    def firstDigit(self, num: int) -> int:
        num_digits = len(str(num))
        return int(str(num)[num_digits - 1])
        
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # list containing addition of l1 and l2 (starts on l3.next)
        l3 = ListNode()
        l1_ptr = l1
        l2_ptr = l2
        l3_ptr = l3
        remainder = 0
        
        # Add along the shared length of l1 and l2
        while (l1_ptr and l2_ptr):
            digit = l1_ptr.val + l2_ptr.val + remainder
            remainder = digit // 10
            
            l3_ptr.next = ListNode(self.firstDigit(digit))
            
            l1_ptr = l1_ptr.next
            l2_ptr = l2_ptr.next
            l3_ptr = l3_ptr.next
            
        # Add any leftover digits from l1 or l2
        while (l1_ptr):
            digit = l1_ptr.val + remainder
            remainder = digit // 10
            
            l3_ptr.next = ListNode(self.firstDigit(digit))
            
            l1_ptr = l1_ptr.next
            l3_ptr = l3_ptr.next
        
        while (l2_ptr):
            digit = l2_ptr.val + remainder
            remainder = digit // 10
            
            l3_ptr.next = ListNode(self.firstDigit(digit))
            
            l2_ptr = l2_ptr.next
            l3_ptr = l3_ptr.next
        
        # Add last carried one if it exists
        if (remainder):
            l3_ptr.next = ListNode(remainder)
        
        return l3.next
