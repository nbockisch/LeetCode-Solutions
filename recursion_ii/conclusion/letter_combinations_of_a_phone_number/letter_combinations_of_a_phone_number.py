'''
Author: Nathan Bockisch
Date: November 13, 2021
'''

class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        # Make sure digits is a valid string
        if (not digits): return []
        
        chars_at = {
            "2" : "abc",
            "3" : "def",
            "4" : "ghi",
            "5" : "jkl",
            "6" : "mno",
            "7" : "pqrs",
            "8" : "tuv",
            "9" : "wxyz"
        }
        combos = []
        
        def getCombos(dig_index: int, cur_str: str):
            # Base case, at end of digits
            if dig_index >= len(digits):
                combos.append(cur_str)
                return
            
            # Get every possible character value at this
            # index of the combination for this index in digits
            for c in chars_at[digits[dig_index]]:
                getCombos(dig_index + 1, cur_str + c)
        
        getCombos(0, "")
        return combos
