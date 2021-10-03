/**
 * Author: Nathan Bockisch
 * Date: October 2, 2021
 **/

class Solution {
public:
    int heightChecker(vector<int>& heights) {
        vector<int> expected = heights;
        sort(expected.begin(), expected.end());
        int err_count = 0;
        
        for (int i = 0; i < heights.size(); i++) {
            if (heights[i] != expected[i])
                err_count++;
        }
        
        return err_count;
    }
};
