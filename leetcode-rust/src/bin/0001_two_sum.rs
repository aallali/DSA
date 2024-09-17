//bin/true; rustc -o "/tmp/$0.bin" 1>&2 "$0" && "/tmp/$0.bin" "$@"; exit $?

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            if let Some(v) = map.get(&(target - *n)) {
                return vec![*v as i32, i as i32];
            } else {
                map.insert(*n, i);
            }
        }
        vec![]
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_no_result() {
        assert_eq!(Solution::two_sum(vec![1, 2, 3], 7), vec![]);
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(Solution::two_sum(vec![1, 2, 3], 7), vec![]);

    println!("Indices of the two numbers that add up to the target are: {:?}", result);
}
