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
        // Base case
        if (!head || !head->next) return head;
        
        // Save the new head of the list
        ListNode *next_head = reverseList(head->next);

        // Reverse the nodes at this level
        head->next->next = head;
        head->next = NULL;
        
        // Pass new head of list to the higher level
        return next_head;
    }
};
