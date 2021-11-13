'''
Author: Nathan Bockisch
Date: November 7, 2021
'''

class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        def checkBox(start: int, end: int) -> int:
            # Base case, we're checking an array of 1 int
            if (start == end): 
                return heights[start]
            elif (start > end):
                return 0
            
            # Get the minimum value, to get the widest box span at this range
            min_index = start
            
            for i in range(start, end + 1):
                if (heights[i] < heights[min_index]):
                    min_index = i
            
            cur_max = heights[min_index] * (end - start + 1)
            
            # Compare with the widest box spans before and after the minimum value's index
            return max(checkBox(start, min_index - 1), checkBox(min_index + 1, end), cur_max)
        
        return checkBox(0, len(heights) - 1)
