// https://leetcode.com/problems/largest-number/
/*
179. Largest Number | Medium

Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.

Since the result may be very large, so you need to return a string instead of an integer.

Example 1:
    Input: nums = [10,2]
    Output: "210"
    Example 2:

    Input: nums = [3,30,34,5,9]
    Output: "9534330"
    
Constraints:
    1 <= nums.length <= 100
    0 <= nums[i] <= 109
*/

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str = nums.into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        nums_str.sort_by(|a,b| b.cmp(a));
        nums_str.join("")
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_number(vec![3,30,34,5,9]), "9534330");
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::largest_number(vec![3,30]), "330");
    }
}

fn main() {
    Solution::largest_number(vec![3,30,34,5,9]);
}
