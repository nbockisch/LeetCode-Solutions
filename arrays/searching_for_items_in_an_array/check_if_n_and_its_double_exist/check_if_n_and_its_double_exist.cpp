/**
 * Author: Nathan Bockisch
 * Date: September 30, 2021
 **/

class Solution {
public:
    bool checkIfExist(vector<int>& arr) {
        unordered_map<int, int> prev_vals;
        
        for (int i = 0; i < arr.size(); i++) {
            // Check if double of arr[i] found before
            if (prev_vals.find(2 * arr[i]) != prev_vals.end()) {
                return true;
            } else if (arr[i] % 2 == 0) {
                if (prev_vals.find(arr[i] / 2) != prev_vals.end()) {
                    return true;
                }
            }
            
            
            prev_vals.insert({arr[i], i});
        }
        
        return false; 
    }
};
