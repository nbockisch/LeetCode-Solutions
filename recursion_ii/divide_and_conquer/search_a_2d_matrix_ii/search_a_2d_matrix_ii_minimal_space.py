'''
Author: Nathan Bockisch
Date: October 27, 2021

Algorithm:
1. Start at the bottom left value
2. If that value is less than the target,
advance through the row by one column
3. If that value is greater than the target,
then all remaining values in that row will be greater,
so move up one row
4. If the value equals the target, return True
5. If you move outside the matrix row or column sizes,
return False
'''

class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        col = 0
        row = len(matrix) - 1
        
        while (col < len(matrix[row]) and row >= 0):
            if matrix[row][col] == target:
                return True
            elif matrix[row][col] < target:
                col += 1
            else:
                row -= 1
        
        return False
