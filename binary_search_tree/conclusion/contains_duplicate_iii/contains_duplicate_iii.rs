/*
 * Author: Nathan Bockisch
 * Date: July 5, 2022
 */
use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut tree = BTreeSet::new();
        for (i, &val) in nums.iter().enumerate() {

            // Make sure the elements added don't get outside the range of k
            if i > (k as usize) {
                tree.remove(&nums[i - (k as usize) - 1]);
            }

            // Check if there's an element <= t when absolute-value subtracted
            // from val
            if let Some(_) = tree.range(val.saturating_sub(t)..=val.saturating_add(t)).next() {
                return true;
            }

            // Insert the current value for future checks
            tree.insert(val);
        }

        false
    }
}
