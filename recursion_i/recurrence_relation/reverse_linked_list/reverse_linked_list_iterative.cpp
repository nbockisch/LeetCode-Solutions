/**
 * Author: Nathan Bockisch
 * Date: October 20, 2021
 **/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        ListNode *cur = head;
        ListNode *cur_next;
        
        while (cur && cur->next) {
            // Save the new head of list
            cur_next = cur->next;
            
            // Reverse the old head with it's next
            cur->next = cur->next->next;
            cur_next->next = head;
            
            // Set the new head
            head = cur_next;
        }
        
        return head;
    }
};
