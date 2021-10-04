/**
 * Author: Nathan Bockisch
 * Date: October 4, 2021
 **/

class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        int beg_ptr = 0;
        int end_ptr = nums.size() - 1;
        vector<int> squares(nums.size());
        
        for (int i = nums.size() - 1; i >= 0; i--) {
            if (abs(nums[beg_ptr]) > abs(nums[end_ptr])) {
                squares[i] = nums[beg_ptr] * nums[beg_ptr];
                beg_ptr++;
                continue;
            }
            
            squares[i] = nums[end_ptr] * nums[end_ptr];
            end_ptr--;
        }
        
        return squares;
    }
};
