struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();

        nums.iter() // <Iter<'_, i32>
            .for_each(|n| {
                *map.entry(*n) // Entry<'_, i32, i32>
                    .or_insert(0) += 1
            });

        let mut tuples: Vec<(i32, i32)> = map.into_iter().collect();
        tuples.sort_by(|a, b| b.1.cmp(&a.1));

        tuples
            .into_iter() // IntoIter<(i32, i32)>
            .take(k as usize) // impl Iterator<Item = (i32, i32)>
            .map(|t| t.0) // impl Iterator<Item = 32>
            .collect::<Vec<i32>>()
    }
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2,2, 3, 3, 4];
    let k = 2;
    let output = vec![1, 2];
    println!("{:#?}", Solution::top_k_frequent(nums.clone(), k));
    assert_eq!(Solution::top_k_frequent(nums, k), output);
    let nums = vec![1, 1, 1, 2, 2, 2, 2, 3, 4];
    let k = 2;
    let output = vec![2, 1];
    assert_eq!(Solution::top_k_frequent(nums, k), output);
    let nums = vec![1, 1, 1, 2, 2, 3, 4];
    let k = 2;
    let output = vec![1, 2];
    assert_eq!(Solution::top_k_frequent(nums, k), output);
}
