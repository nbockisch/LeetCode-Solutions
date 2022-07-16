/*
 * Author: Nathan Bockisch
 * Date: July 11, 2022
 */
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let max: i32 = *nums.iter().max().unwrap();
        let mut max_index = 0;

        for i in 0..nums.len() {
            let cur = nums[i];

            if cur == max {max_index = i;}
            else if (2 * cur) > max {return -1;}
        }

        max_index as i32
    }
}
