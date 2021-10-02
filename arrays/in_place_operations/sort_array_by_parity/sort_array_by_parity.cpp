/**
 * Author: Nathan Bockisch
 * Date: October 2, 2021
 **/

class Solution {
public:
    // Helper function to efficiently check parity
    bool isEven(int num) {
        if ((num & 1) == 0) {
            return true;
        }
        
        return false;
    }
    
    vector<int> sortArrayByParity(vector<int>& nums) {
        int beg_ptr = 0;
        int end_ptr = nums.size() - 1;
        
        while (beg_ptr < end_ptr) {
            if (!isEven(nums[beg_ptr]) && isEven(nums[end_ptr])) {
                int tmp = nums[end_ptr];
                nums[end_ptr--] = nums[beg_ptr];
                nums[beg_ptr++] = tmp;
            } else if (isEven(nums[end_ptr])) {
                beg_ptr++;
            } else if (!isEven(nums[beg_ptr])) {
                end_ptr--;
            } else {
                beg_ptr++;
                end_ptr--;
            }
        }
        
        return nums;
    }
};
