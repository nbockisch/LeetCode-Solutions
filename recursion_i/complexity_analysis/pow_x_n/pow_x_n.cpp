/**
 * Author: Nathan Bockisch
 * Date: October 22, 2021
 **/

class Solution {
public:
    double myPow(double x, long long n) {
        if (n == 0) // Base case
            return 1;
        else if (n < 0) { // Adjust for negative exponent
            n *= -1;
            x = 1 / x;
        }
        
        /* Instead of calculating n at every step,
        just get n / 2 and square it */
        double half = myPow(x, n / 2);
        if (n % 2 == 0)
            return half * half;
        else
            return half * half * x;
    }
};
