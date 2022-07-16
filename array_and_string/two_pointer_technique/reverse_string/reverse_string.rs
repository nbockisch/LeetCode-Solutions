/*
 * Author: Nathan Bockisch
 * Date: July 12, 2022
 */
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut l_ptr, mut r_ptr) = (0, s.len() - 1);

        while (r_ptr > l_ptr) {
            s.swap(l_ptr, r_ptr);
            l_ptr += 1;
            r_ptr -= 1;
        }
    }
}
