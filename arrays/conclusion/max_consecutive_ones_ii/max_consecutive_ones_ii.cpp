/**
 * Author: Nathan Bockisch
 * Date: October 3, 2021
 **/

class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        int slow_ptr = 0;
        int zeros = 0;
        int cur_max = 0;
        
        for (int i = 0; i < nums.size(); i++) {
            if (!nums[i])
                zeros++;
            
            while (zeros > 1) {
                if (!nums[slow_ptr++])
                    zeros--;
            }
            
            /* current streak is the current index (adjusted for 0 indexing)
            minus the last zero location */
            cur_max = max(cur_max, i + 1 - slow_ptr);
        }
        
        return cur_max;
    }
};
