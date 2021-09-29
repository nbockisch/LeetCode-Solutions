/**
 * Author: Nathan Bockisch
 * Date: September 29, 2021
 **/

class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int nums1_ptr = m - 1;
        int nums2_ptr = n - 1;
        int out_ptr = m + n - 1;
        
        // Perform the final step of merge sort
        while (nums1_ptr >= 0 && nums2_ptr >= 0) {
            if (nums1[nums1_ptr] < nums2[nums2_ptr]) {
                nums1[out_ptr--] = nums2[nums2_ptr--];
                continue;
            }
            
            nums1[out_ptr--] = nums1[nums1_ptr--];
        }
        
        while (nums1_ptr >= 0) {
            nums1[out_ptr--] = nums1[nums1_ptr--];
        }
        
        while (nums2_ptr >= 0) {
            nums1[out_ptr--] = nums2[nums2_ptr--];
        }
    }
};
