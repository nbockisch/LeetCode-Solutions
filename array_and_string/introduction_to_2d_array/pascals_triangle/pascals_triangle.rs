/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows {
            let mut level: Vec<i32> = Vec::new();

            if let Some(prev_level) = answer.get((i - 1) as usize) {
                let mut prev_parent: Option<i32> = None;

                for j in 0..=prev_level.len() {
                    let cur_parent = match prev_level.get(j as usize) {
                        Some(val) => Some(*val),
                        None => None,
                    };

                    let cur_val = match (prev_parent, cur_parent) {
                        (None, None) => 1,
                        (None, Some(cur)) => cur,
                        (Some(prev), Some(cur)) => prev + cur,
                        (Some(prev), None) => prev
                    };

                    prev_parent = cur_parent;
                    level.push(cur_val);
                }
            } else {
                level.push(1);
            }

            answer.push(level);
        }

        answer
    }
}
