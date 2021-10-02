/**
 * Author: Nathan Bockisch
 * Date: October 2, 2021
 **/

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int new_end = 0;
        
        for (int i = 0; i < nums.size(); i++) {
            /* since the array is sorted, we know a number is unique if
            it's greater than the last one.*/
            if (i < 1 || nums[i] > nums[i - 1])
                nums[new_end++] = nums[i];
        }
        
        return new_end;
    }
};
