/**
 * Author: Nathan Bockisch
 * Date: September 30, 2021
 **/

class Solution {
public:
    vector<int> replaceElements(vector<int>& arr) {
        int max_val = -1;
        int tmp;
        
        for (int end_ptr = arr.size() - 1; end_ptr >= 0; end_ptr--) {
            int tmp = arr[end_ptr];
            arr[end_ptr] = max_val;
            max_val = max(max_val, tmp);
        }
        
        return arr;
    }
};
