// 1. Two Sum
// link: https://leetcode.com/problems/two-sum/description/

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32>= HashMap::new();
        let mut ans = vec![]; 

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) {
                ans.push(map[&complement]);
                ans.push(i as i32);
                break;
            }
            map.insert(nums[i], i as i32);
        }
        ans
    }
}

fn main() {
    let input = vec![2, 7, 11, 15];
    let result = Solution::two_sum(input, 9);
    println!("{:?}", result);
}