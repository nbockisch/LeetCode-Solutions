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
    # Iterative linked list reversal
    def reverseList(self, head: Optional[ListNode]) -> ListNode:
        ptr_a, ptr_b = head, head.next
        
        while (ptr_b):
            ptr_a.next = ptr_b.next
            ptr_b.next = head
            head = ptr_b
            ptr_b = ptr_a.next
        
        return head
    
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        slow, fast = head, head
        
        # Get slow to middle of list
        while (fast and fast.next):
            slow = slow.next
            fast = fast.next.next
        
        # Reverse the middle to end part of list
        slow = self.reverseList(slow)
        
        # If beginning to middle, and reversed middle to end lists
        # are identical, there is a palindrome
        fast = head
        while (slow):
            if slow.val != fast.val:
                return False
            
            slow = slow.next
            fast = fast.next
            
        return True
