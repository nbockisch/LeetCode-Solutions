'''
Author: Nathan Bockisch
Date: November 3rd
'''

class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        def getCombo(combo: list, cur: int):
            # If we're at the last level, 
            # add the current combination to the output
            if len(combo) == k:
                combos.append(combo[:])
            
            # Backtrack combination to get every possible
            # digit at this level
            for i in range(cur, n + 1):
                combo.append(i)
                getCombo(combo, i + 1)
                combo.pop()
                
        combos = []
        getCombo([], 1)
        return combos
