/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        int count = 0, max_count = 0;

        for (int num : nums) {
            count += num;

            if (!num) {
                if (count > max_count)
                    max_count = count;
                count = 0;
                continue;
            }
        } 
        
        if (count > max_count)
            return count;
        
        return max_count;
    }
};
