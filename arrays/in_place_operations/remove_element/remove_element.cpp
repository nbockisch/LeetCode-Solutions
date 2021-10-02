/**
 * Author: Nathan Bockisch
 * Date: October 2, 2021
 **/

class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int new_len = 0;
        
        for (int num : nums) {
            if (num == val) {
                continue;
            }
            nums[new_len++] = num;
        }
        
        return new_len;
    }
};
