/**
 * Author: Nathan Bockisch
 * Date: September 30, 2021
 **/

class Solution {
public:
    bool validMountainArray(vector<int>& arr) {
        if (arr.size() < 3)
            return false;
        
        // find point when strictly increasing stops
        int i = 0;
        while (arr[i] < arr[i + 1]) {
            i++;
        }
        
        // make sure we're not at the index extremes
        if (i >= arr.size() - 1 || i == 0) {
            return false; 
        }
        
        // verify the array ends strictly decreasing
        while (i < arr.size() - 1) {
            if (arr[i] <= arr[i + 1]) {
                return false;
            }
            i++;
        }
        return true;
    }
};
