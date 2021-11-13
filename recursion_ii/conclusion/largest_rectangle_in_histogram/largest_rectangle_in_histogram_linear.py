'''
Author: Nathan Bockisch
Date: November 13, 2021
'''

class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        vals = deque()
        max_area = 0
        
        # Make sure we know the end of the stack
        vals.append(-1)
        
        for i in range(len(heights)):
            '''
            If the current value is greater than the last, we found the righthand
            bound of the last element. Therefore we calculate the max area possible
            from the last area. This is done by taking the height of the last area,
            and multiplying it by the width. The width is the difference in index
            between the righthand boundary and the lefthand boundary.
            '''
            while (vals[-1] != -1 and heights[i] < heights[vals[-1]]):
                height = heights[vals.pop()]
                width = i - vals[-1] - 1
                max_area = max(max_area, width * height)
            
            vals.append(i)
        
        '''
        Finally, calculate the max possible area of the minimum elements leftover
        in the stack. This is the height of these value times the width, which is the
        difference between the index of the previous stack element (which is the left boundary)
        and the end of the list, since no righthand boundary exists at this point.
        '''
        while (vals[-1] != -1):
            height = heights[vals.pop()]
            width = len(heights) - vals[-1] - 1
            max_area = max(max_area, width * height)
            
        return max_area
