/*
 * Author: Nathan Bockisch
 * Date: July 11, 2022
 */
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let last_i = digits.len() - 1;

        if (digits[last_i] + 1) <= 9 {
            digits[last_i] += 1;
            return digits;
        }

        for i in (0..=last_i).rev() {
            match digits[i] + 1 > 9 {
                true => {
                    if i == 0 {
                        digits[i] = 1;
                        digits.push(0);
                    } else {digits[i] = 0;}
                },
                false => {
                    digits[i] += 1;
                    break;
                },
            };
        }

        digits
    }
}
