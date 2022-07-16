/*
 * Author: Nathan Bockisch
 * Date: July 15, 2022
 */
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut answer = vec![1; row_index + 1];

        for i in 1..row_index {
            let mut j = i;
            while (j > 0) {
                answer[j] += answer[j - 1];
                j -= 1;
            }
        }

        answer
    }
}
