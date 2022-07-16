/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut max_count = 0;

        nums.chunks(2).for_each(| nums | {
            max_count += nums[0];
        });

        max_count
    }
}
