/*
 * Author: Nathan Bockisch
 * Date: July 11, 2022
 */
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let mut lsum = 0;

        for i in 0..nums.len() {
            sum -= nums[i];
            if (sum == lsum) { return i as i32; }
            lsum += nums[i];
        }

        -1
    }
}
