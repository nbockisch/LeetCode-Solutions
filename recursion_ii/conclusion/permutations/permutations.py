'''
Author: Nathan Bockisch
Date: November 13, 2021
'''

class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        perms = []
        
        def get_permutations(first: int):
            # Base case, we're at the last digit
            if (first == len(nums)):
                perms.append(nums[:])
                return
            
            # Get permutations where nums[first] is every
            # possible value from the rest of the array
            for i in range(first, len(nums)):
                # Make nums[first] every possible value
                nums[first], nums[i] = nums[i], nums[first]
                get_permutations(first + 1)
                
                # Backtrack and reset it so the next permutation can be tried
                nums[first], nums[i] = nums[i], nums[first]
                
        get_permutations(0)
        return perms
