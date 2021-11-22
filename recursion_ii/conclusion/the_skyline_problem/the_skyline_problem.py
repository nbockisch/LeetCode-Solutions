'''
Author: Nathan Bockisch
Date: November 22, 2021
'''

class Solution:
    def mergeSkylines(self, left: List[List[int]], right: List[List[int]]) -> List[List[int]]:
        # Keep track of index in left and right lists, and the current Y at each point
        point_l = 0
        point_r = 0
        left_y = right_y = cur_y = 0
        cur_x = 0
        
        output = []
        
        while (point_l < len(left) and point_r < len(right)):
            # Get the minimum x value (furthest left building) and move the pointer
            if (left[point_l][0] < right[point_r][0]):
                cur_x, left_y = left[point_l]
                point_l += 1
            else:
                cur_x, right_y = right[point_r]
                point_r += 1
            
            # Find the tallest building between the left and right lists
            max_y = max(left_y, right_y)
            
            # If the height changed since the last step, add the new point to the output
            if (cur_y != max_y):
                '''
                Update the height of the last building if this one is taller and overlaps,
                otherwise append this new point of the tallest height and furthest left
                building between the left and right
                '''
                if (output and output[-1][0] == cur_x):
                    output[-1][1] = max_y
                else:
                    output.append([cur_x, max_y])
                
                cur_y = max_y
                
        '''
        Append any leftovers in the left or right lists, 
        following the same output logic as above
        '''
        while (point_l < len(left)):
            cur_x, y = left[point_l]
            if (cur_y != y):
                # Update output
                if (output and output[-1][0] == cur_x):
                    output[-1][1] = y
                else:
                    output.append([cur_x, y])
                
                cur_y = y
            
            point_l += 1
            
        while (point_r < len(right)):
            cur_x, y = right[point_r]
            if (cur_y != y):
                # Update output
                if (output and output[-1][0] == cur_x):
                    output[-1][1] = y
                else:
                    output.append([cur_x, y])
                
                cur_y = y
            
            point_r += 1
            
        return output
            
        
    def getSkyline(self, buildings: List[List[int]]) -> List[List[int]]:
        # Base case, we have no buildings or only one
        if (len(buildings) == 0):
            return []
        elif (len(buildings) == 1):
            # Return skyline for one building
            return [[buildings[0][0], buildings[0][2]], 
                    [buildings[0][1], 0]]
        
        mid = len(buildings) // 2
        
        # Merge the left and right half sides of the skyline, divide and conquer style
        left = self.getSkyline(buildings[:mid])
        right = self.getSkyline(buildings[mid:])
        
        return self.mergeSkylines(left, right)
