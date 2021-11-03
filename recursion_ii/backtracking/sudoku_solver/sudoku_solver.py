'''
Author: Nathan Bockisch
Date: November 3, 2021
'''

class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        
        # Return the next empty row and column as a tuple
        def findNextFree() -> tuple:
            for i in range(0, 9):
                for j in range(0, 9):
                    if board[i][j] == ".":
                        return (i, j)
            
            return (None, None)
        
        # Check if a number can go in a given board position
        def isValid(row: int, col: int, num: int) -> bool:
            # Check column
            for i in range(9):
                if board[row][i] == str(num):
                    return False
            
            # Check row
            for i in range(9):
                if board[i][col] == str(num):
                    return False
            
            # Check 3x3 square
            sq_row = 3 * (row // 3)
            sq_col = 3 * (col // 3)
            
            for i in range(sq_row, sq_row + 3):
                for j in range(sq_col, sq_col + 3):
                    if board[i][j] == str(num):
                        return False
                    
            return True
        
        def solve(pos: tuple) -> bool:
            # Check every possible solution
            for i in range(1, 10):
                if isValid(pos[0], pos[1], i):
                    board[pos[0]][pos[1]] = str(i)
                    
                    next_pos = findNextFree()
                    
                    # Check if the puzzle is solved here or at the next pos,
                    # otherwise reset and try the next number
                    if next_pos[0] == None or solve(next_pos):
                        return True
                    else:
                        board[pos[0]][pos[1]] = '.'
                  
            # If no valid answers found at this pos, backtrack
            return False
        
        # solve the puzzle
        next_pos = findNextFree()
        
        # Make sure the puzzle isn't already solved
        if (next_pos[0] != None):
            solve(next_pos)
        
