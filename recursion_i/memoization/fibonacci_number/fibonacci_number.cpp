/**
 * Author: Nathan Bockisch
 * Date: October 22, 2021
 **/

class Solution {
public:
    unordered_map<int, int> cache;
    
    int fib(int n) {
        // Avoid duplicate calculations
        if (cache.find(n) != cache.end()) return cache[n];
        
        int value = n;
        
        if (n > 1) {
            value = fib(n - 1) + fib(n - 2);
        }
        
        // Avoid duplicate calculations in the future
        cache[n] = value;
        
        return value;
    }
};1
