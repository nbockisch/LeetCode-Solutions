'''
Author: Nathan Bockisch
Date: November 3, 2021
'''
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        def getCombinations(combo: str, n_open: int, n_closed: int):
            # Base case, when we've generated the right length string, return
            if (len(combo) == 2 * n):
                combos.append(combo)
                return
            
            # Use backtracking to match every open parenthesis to a closed one
            if (n_open < n):
                getCombinations(combo + "(", n_open + 1, n_closed)
            
            if (n_closed < n_open):
                getCombinations(combo + ")", n_open, n_closed + 1)
                
        combos = []
        getCombinations("", 0, 0)
        return combos
