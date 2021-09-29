/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    // Helper function
    int get_digits(int num) {
        int digits = 1;

        while (num /= 10)
            digits++;

        return digits;
    }
    
    int findNumbers(vector<int>& nums) {
        int count = 0;
        for (int num : nums) {
            if (get_digits(num) % 2 == 0)
                count++;
        }
         
        return count;
    }
};
