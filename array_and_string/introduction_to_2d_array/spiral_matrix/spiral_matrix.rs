/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
use std::cmp;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = Vec::new();
        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;

        let (mut xpos, mut ypos): (i32, i32) = (0, 0);
        let mut i = 0;
        loop {
            if xpos > (n - i) as i32 { break; }
            while xpos <= (n - i) as i32 {
                answer.push(matrix[ypos as usize][xpos as usize]);
                xpos += 1;
            }
            xpos = cmp::min(xpos, (n - i) as i32);
            ypos += 1;

            if ypos > (m - i) as i32 { break; }
            while ypos <= (m - i) as i32 {
                answer.push(matrix[ypos as usize][xpos as usize]);
                ypos += 1;
            }
            ypos = cmp::min(ypos, (m - i) as i32);
            xpos -= 1;

            if xpos < (0 + i) as i32 { break; }
            while xpos >= (0 + i) as i32 {
                answer.push(matrix[ypos as usize][xpos as usize]);
                xpos -= 1;
            }
            xpos = cmp::max(xpos, (0 + i) as i32);
            ypos -= 1;

            i += 1;

            if ypos < (0 + i) as i32 { break; }
            while ypos >= (0 + i) as i32 {
                answer.push(matrix[ypos as usize][xpos as usize]);
                ypos -= 1;
            }
            ypos = cmp::max(ypos, (0 + i) as i32);
            xpos += 1;
        }

        answer
    }
}
