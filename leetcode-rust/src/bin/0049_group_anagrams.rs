// https://leetcode.com/problems/contains-duplicate/description/

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let key = Solution::get_anag_key(s.clone());
            map.entry(key).or_insert_with(Vec::new).push(s);
        }

        map.into_values().collect()
    }
    fn get_anag_key(s: String) -> Vec<u8> {
        let mut s_sorted = s.clone().into_bytes();
        s_sorted.sort_unstable();
        s_sorted
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let input: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|w| w.to_string())
            .collect();
        let output: Vec<Vec<String>> =
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
                .into_iter()
                .map(|inner_vec| inner_vec.into_iter().map(|s| s.to_string()).collect())
                .collect();

        assert_eq!(Solution::group_anagrams(input), output);
    }
}

fn main() {
    let input: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(|w| w.to_string())
        .collect();
    let _output: Vec<Vec<String>> =
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
            .into_iter()
            .map(|inner_vec| inner_vec.into_iter().map(|s| s.to_string()).collect())
            .collect();

    Solution::group_anagrams(input);
}
