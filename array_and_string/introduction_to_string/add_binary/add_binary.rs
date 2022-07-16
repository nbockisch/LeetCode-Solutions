/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_num = u128::from_str_radix(&a, 2).unwrap();
        let b_num = u128::from_str_radix(&b, 2).unwrap();

        format!("{:b}", a_num + b_num)
    }
}
