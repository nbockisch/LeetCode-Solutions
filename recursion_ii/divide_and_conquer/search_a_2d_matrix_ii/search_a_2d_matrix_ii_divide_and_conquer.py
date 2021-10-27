'''
Author: Nathan Bockisch
Date: October 27, 2021
'''

class Solution:
    def dcSearch(self, matrix: List[List[int]], left: int, bottom: int, right: int, top: int, target: int) -> bool:
        # Base cases, matrix invalid or target outside matrix value range
        if left > right or bottom > top:
            return False
        elif target < matrix[bottom][left] or target > matrix[top][right]:
            return False;
        
        # Get the middle of the matrix
        mid = left + (right - left) // 2
        row = bottom
        
        # Find the row where the middle exceeds the target
        while (row <= top and matrix[row][mid] <= target):
            if matrix[row][mid] == target:
                return True
            row += 1
        
        '''
        Search the two submatrices where:
        1. Where the columns up to the middle are <= target, this is from the row calculated
        previously to the last row in the matrix
        2. Where the columns after the middle are <= target, this is from the first row up to the row
        calculated previously
        ''' 
        return self.dcSearch(matrix, left, row, mid - 1, top, target) or self.dcSearch(matrix, mid + 1, bottom, right, row - 1, target)
        
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        return self.dcSearch(matrix, 0, 0, len(matrix[0]) - 1, len(matrix) - 1, target)
