/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() { return String::from(""); }

        strs.iter().skip(1).fold(strs[0].clone(), |first, next| {
            first
                .chars()
                .zip(next.chars())
                .take_while(|(f_char, n_char)| f_char == n_char)
                .map(|(ch, _) | ch)
                .collect()
        })
    }
}
