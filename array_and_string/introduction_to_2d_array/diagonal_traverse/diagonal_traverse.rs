/*
* Author: Nathan Bockisch
* Date: July 12, 2022
*/
use std::cmp;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = Vec::new();
        let m = mat.len() - 1;
        let n = mat[0].len() - 1;

        for diag in 0..=(m + n) {
            let mut xpos = cmp::min(diag, n);
            let mut ypos = if (diag <= n) { 0 } else { diag - n };

            let mut diag_row = Vec::new();
            while xpos >= 0 && xpos <= n && ypos >= 0 && ypos <= m {
                diag_row.push(mat[ypos][xpos]);
                ypos += 1;
                xpos -= 1;
            }

            if (diag % 2 == 0) { diag_row.reverse() }
            answer.append(&mut diag_row);
        }

        answer
    }
}
