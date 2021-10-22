/**
 * Author: Nathan Bockisch
 * Date: October 22, 2021
 **/

class Solution {
public:
    unordered_map<int, int> cache;
    
    int takeStep(int n, int steps) {
        int count = 0;
        int diff = n - steps;
        
        // Avoid duplicate calculations
        if (cache.find(diff) != cache.end()) return cache[diff];
        
        // Base case
        if (diff == 0)
            count++;
        else if (diff > 0) {
            count += takeStep(diff, 1);
            count += takeStep(diff, 2);
        }
        
        cache[diff] = count;
        return count;
    }
    
    int climbStairs(int n) {
        return takeStep(n, 1) + takeStep(n, 2);
    }
};
