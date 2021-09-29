/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        vector<int> nums_out(nums.size()); 
        int out_index = nums.size() - 1;
        int end_ptr = nums.size() - 1;
        int beg_ptr = 0;  
        
        // Sort squared elements into output array
        while (beg_ptr <= end_ptr) {
            // value at end pointer greater than that at beginning pointer, so insert it
            if (abs(nums[beg_ptr]) < abs(nums[end_ptr])) {
                nums_out[out_index] = nums[end_ptr] * nums[end_ptr];
                out_index--;
                end_ptr--;
                continue;
            }
            
            // value at beginning pointer greater than that at end pointer, so insert it
            nums_out[out_index] = nums[beg_ptr] * nums[beg_ptr];
            out_index--;
            beg_ptr++;
        } 
        
        return nums_out;
    }
};
