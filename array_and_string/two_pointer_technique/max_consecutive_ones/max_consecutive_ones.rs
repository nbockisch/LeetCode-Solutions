/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
use std::cmp;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut cur_count, mut max_count) = (0, 0);

        for num in nums {
            match num {
                1 => cur_count += 1,
                _ => {
                    max_count = cmp::max(cur_count, max_count);
                    cur_count = 0;
                }
            }
        }

        cmp::max(cur_count, max_count)
    }
}
