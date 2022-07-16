/*
 * Author: Nathan Bockisch
 * Date: July 14, 2022
 */
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let mut start_pos = 0;
        let mut cur_i = 0;

        while cur_i < nums.len() {
            let mut last_val = nums[start_pos];
            let mut cur_pos = start_pos;
            loop {
                let next_pos = (cur_pos + k) % nums.len();

                let tmp = nums[next_pos];
                nums[next_pos] = last_val;
                last_val = tmp;

                cur_pos = next_pos;
                cur_i += 1;

                if (start_pos == cur_pos) { break; }
            }
            start_pos += 1;
        }
    }
}
