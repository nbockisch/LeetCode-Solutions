/**
 * Author: Nathan Bockisch
 * Date: October 4, 2021
 **/

class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        unordered_map<int, int> found_nums;
        vector<int> not_found;
        
        // Use a map to keep track of all unique values in the array
        for (int i = 0; i < nums.size(); i++)
            if (found_nums.find(nums[i]) == found_nums.end())
                found_nums[nums[i]] = i;
        
        // Find and return the numbers not found
        for (int i = 1; i <= nums.size(); i++)
            if (found_nums.find(i) == found_nums.end())
                not_found.push_back(i);
        
        return not_found;
    }
};
