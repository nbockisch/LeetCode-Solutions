/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    void duplicateZeros(vector<int>& arr) {
        for (int i = 0; i < arr.size(); i++) {
            if (arr[i] == 0) {
                arr.insert(arr.begin() + i, 0);
                arr.pop_back();
                // Skip the index of the 0 that just got pushed right, to avoid duplicating twice
                i++;
            }
        }
    }
};
