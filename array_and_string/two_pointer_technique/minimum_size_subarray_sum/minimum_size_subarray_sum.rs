/*
 * Author: Nathan Bockisch
 * Date: July 13, 2022
 */
use std::cmp;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut min, mut l_ptr, mut cur_sum) = (0, 0, 0);

        for i in 0..nums.len() {
            cur_sum += nums[i];

            while cur_sum >= target {
                let cur_min = i + 1 - l_ptr;
                if min == 0 {
                    min = cur_min;
                } else {
                    min = cmp::min(cur_min, min);
                }

                cur_sum -= nums[l_ptr];
                l_ptr += 1;
            }
        }

        min as i32
    }
}
