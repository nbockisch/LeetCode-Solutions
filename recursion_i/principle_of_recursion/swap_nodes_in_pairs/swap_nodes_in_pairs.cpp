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
    ListNode* swapPairs(ListNode* head) {
        // Base case
        if (!head || !head->next) return head;
        
        // Save the pointer to the next pair to swap
        ListNode *next_pair = head->next->next;
        
        // Swap the nodes and connect them to the next pair of swapped nodes
        head->next->next = head;
        head = head->next;
        head->next->next = swapPairs(next_pair);
        
        return head;
    }
};
