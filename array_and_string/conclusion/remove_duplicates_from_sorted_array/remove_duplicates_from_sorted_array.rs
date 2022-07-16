/*
 * Author: Nathan Bockisch
 * Date: July 15, 2022
 */
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;

        for cur in 0..nums.len() {
            if nums[cur] != nums[k] {
                k += 1;
                nums[k] = nums[cur];
            }
        }

        (k + 1) as i32
    }
}
