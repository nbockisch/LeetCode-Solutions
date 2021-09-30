/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        if (nums.empty()) {
            return 0;
        }
        
        int new_len = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] != nums[new_len]) {
                new_len++;
                nums[new_len] = nums[i];
            }
        }        
        
        return new_len + 1;
    }
};
