/*
 * Author: Nathan Bockisch
 * Date: July 15, 2022
 */
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}
