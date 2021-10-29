'''
Author: Nathan Bockisch
Date: October 29, 2021
'''
class Solution:
    def isValid(self, row: int, col: int, queens: list) -> bool:
        # Get diagonal ratios
        pos_diag = row + col
        neg_diag = row - col
        
        for queen in queens:
            # Get diagonal ratios for this queen
            qpos_diag = queen[0] + queen[1]
            qneg_diag = queen[0] - queen[1]
            
            # Check columns and diagonals (we will never have two on the same row)
            if queen[1] == col or pos_diag == qpos_diag or neg_diag == qneg_diag:
                return False
        
        return True
    
    def countSolutions(self, row: int, n: int, queens: list) -> int:
        count = 0
        
        for col in range(n):
            if self.isValid(row, col, queens):
                # If in the last row, a solution is found
                if row == n - 1:
                    count += 1
                    break
                
                # Otherwise, if the square is valid, check the next row
                queens.append((row, col))
                count += self.countSolutions(row + 1, n, queens)
                
                # Reset and continue looking for solutions
                queens.pop()
        
        return count
        
    def totalNQueens(self, n: int) -> int:
        # Queen positions are stored as a tuple (row, col)
        queens = []
        return self.countSolutions(0, n, queens)
