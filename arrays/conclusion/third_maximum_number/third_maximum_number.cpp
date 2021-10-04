/**
 * Author: Nathan Bockisch
 * Date: October 4
 **/

class Solution {
public:
    int thirdMax(vector<int>& nums) {
        // index 0 is first max, 1 is second max, 2 is third max
        long max_nums[3] = {LONG_MIN, LONG_MIN, LONG_MIN};
        
        for (int num : nums) {
            if (num == max_nums[0] || num == max_nums[1] || num == max_nums[2])
                continue;
            
            if (num > max_nums[0]) {
                max_nums[2] = max_nums[1];
                max_nums[1] = max_nums[0];
                max_nums[0] = num;
            } else if (num > max_nums[1]) {
                max_nums[2] = max_nums[1];
                max_nums[1] = num;
            } else if (num > max_nums[2]) {
                max_nums[2] = num;
            }
        }
        
        if (max_nums[2] != LONG_MIN) {
            return max_nums[2];
        }
        
        return max_nums[0];
    }
};
