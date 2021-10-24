/**
 * Author: Nathan Bockisch
 * Date: October 23, 2021
 **/

/*
The grammar repeats itself, so that index k
is the opposite of index mid + k, therefore
we can return the opposite value depending
on if k is in the first or second half of
the row

0 1
01 2
0110 4
01101001 8
0110100110010110
*/

class Solution {
public:
    int kthGrammar(int n, int k) {
        // Base case
        if (n == 1 && k == 1) 
            return 0;
        
        int mid = pow(2, n - 2);
        
        return (k <= mid) ? kthGrammar(n - 1, k) : !kthGrammar(n - 1, k - mid);
    }
};
