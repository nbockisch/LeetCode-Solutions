/**
 * Author: Nathan Bockisch
 * Date: October 21, 2021
 *
 * Explanation:
 * Pascal's triangle is an array of binomial coefficients,
 * a value at row a and col b is a! / b!(a - b)!. Binomial
 * coefficients have an additive property, so the value
 * (a, b) = (a - 1 / b - 1) + (a - 1 / b). SUccessive binomial 
 * coefficients differ by (a! / b!(a - b)!) / (a! / (b - 1!)(a - b + 1)!) 
 * = (a = b + 1) / b
 *
 * Therefore each element in the row is the previous element multiplied by 
 * this coefficient.
 **/

class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> row = {1};
        
        for (int i = 1; i <= rowIndex; i++) 
            row.push_back((int)((row.back() * (long long)(rowIndex - i + 1)) / i));        
        
        return row;
    }
};
