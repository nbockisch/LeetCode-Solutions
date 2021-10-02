/**
 * Author: Nathan Bockisch
 * Date: October 2, 2021
 **/

class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int beg_ptr = 0;
        
        // Remove zeros and maintian relative order
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i]) {
                nums[beg_ptr++] = nums[i];
            }
        }
        
        // Write zeros back in to the end of array
        for (; beg_ptr < nums.size(); nums[beg_ptr++] = 0);
    }
};
