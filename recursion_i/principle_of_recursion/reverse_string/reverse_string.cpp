/**
 * Author: Nathan Bockisch
 * Date: October 20th, 2021
 **/

class Solution {
public:
    void recursiveReverser(vector<char>& s, int start, int end) {
        // Base case
        if (end <= start) return;
        
        char tmp = s[start];
        s[start] = s[end];
        s[end] = tmp;
        
        // Swap the ith and size - ith characters
        recursiveReverser(s, ++start, --end);
    }
    
    void reverseString(vector<char>& s) {
        recursiveReverser(s, 0, s.size() - 1);
    }
};
