// https://leetcode.com/problems/contains-duplicate/description/

struct Solution;
use std::collections::HashSet;

impl Solution {
    // faster
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::<i32>::new();
        return nums.into_iter().any(|x| !set.insert(x));
    }
    // pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    //     let mut set = HashSet::<i32>::new();
    //     for n in nums {
    //         if set.contains(&n) {
    //             return true;
    //         }
    //         set.insert(n);
    //     }
    //     false
    // }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}

fn main() {
    Solution::contains_duplicate(vec![1, 2, 3, 1]);
}
