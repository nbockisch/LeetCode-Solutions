'''
Author: Nathan Bockisch
Date: October 27, 2021
'''

class Solution:
    def sortArray(self, nums: List[int]) -> List[int]:
        # Base case
        if len(nums) <= 1:
            return nums
        
        mid = len(nums) // 2
        left = self.sortArray(nums[:mid])
        right = self.sortArray(nums[mid:])
        
        # Merge lists
        i, j, k = 0, 0, 0
        
        while (i < len(left) and j < len(right)):
            if left[i] <= right[j]:
                nums[k] = left[i]
                i += 1
            else:
                nums[k] = right[j]
                j += 1
            
            k += 1
        
        while (i < len(left)):
            nums[k] = left[i]
            i += 1
            k += 1
            
        while (j < len(right)):
            nums[k] = right[j]
            j += 1
            k += 1
            
        return nums
