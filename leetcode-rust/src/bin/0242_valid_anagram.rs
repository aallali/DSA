// https://leetcode.com/problems/valid-anagram/

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        Solution::solution2(s, t)
        // Solution::solution1(s, t)
    }
    fn solution2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }
        let mut alpha: [i32; 26] = [0; 26];
        for c in s.chars() {
            alpha[c as usize - 97] += 1;
        }
        for c in t.chars() {
            let char_code = c as usize - 97;
            if alpha[char_code] == 0 {
                return false
            }
            alpha[char_code] -= 1;
        }
        true
    }
    fn solution1(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }
        Solution::get_anag_key(s) == Solution::get_anag_key(t)
    }
    fn get_anag_key(s: String) -> String {
        let mut s_sorted = s.chars().collect::<Vec<char>>();
        s_sorted.sort();
        s_sorted.iter().collect()
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
        assert_eq!(
            Solution::is_anagram(String::from("ab"), String::from("a")),
            false
        );
    }
}

fn main() {
    Solution::is_anagram(String::from("anagram"), String::from("nagaram"));
}
