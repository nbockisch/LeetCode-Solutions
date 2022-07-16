/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            None => -1,
            Some(index) => index as i32,
        }
    }
}
