struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            println!("l:{l}- r:{r}");

            let mid_idx = (l as usize + r as usize) >> 1;
            let mid = nums[mid_idx as usize];
            println!("l:{l}- r:{r}- mid_idx:{mid_idx}- mid:{mid}");

            if mid < target {
                l = mid_idx + 1
            } else if mid > target {
                r = mid_idx - 1
            } else {
                return mid_idx as i32;
            }
        }

        l as i32
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1, 2, 4, 5, 6], 3)); // 2
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 7)); // 4
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 0)); // 0
    
}
