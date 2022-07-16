/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l_ptr, mut r_ptr) = (0, numbers.len() - 1);

        while (l_ptr < r_ptr) {
            let cur_sum = numbers[l_ptr] + numbers[r_ptr];
            let cur_cmp = cur_sum.cmp(&target);

            match cur_cmp {
                Ordering::Equal => break,
                Ordering::Greater => r_ptr -= 1,
                Ordering::Less => l_ptr += 1,
            };
        }

        vec![(l_ptr + 1) as i32, (r_ptr + 1) as i32]
    }
}
