/*
 * Author: Nathan Bockisch
 * Date: July 15, 2022
 */
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut slow, mut fast) = (0, 1);

        while fast < nums.len() {
            if nums[slow] == 0 && nums[fast] != 0 {
                nums.swap(slow, fast);
            }

            if nums[slow] != 0 { slow += 1 }
            fast += 1;
        }
    }
}
