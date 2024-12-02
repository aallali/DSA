struct Solution;


impl Solution {
    pub fn clear_digits(s: String) -> String {
        use std::collections::HashSet;
        let alpha: HashSet<char> = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']
            .iter()
            .cloned()
            .collect();
        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            if alpha.contains(&c) {
                res.pop();
            } else {
                res.push(c);
            }
        }

        res.iter().collect()
    }
}

fn main() {
    println!("{}", Solution::clear_digits("ab34s".to_string()));
}
