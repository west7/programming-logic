// 3005. Count elements with maximum frequency
// link: https://leetcode.com/problems/count-elements-with-maximum-frequency/description/?envType=daily-question&envId=2024-07-21
struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let max_freq = *map.values().max().unwrap_or(&0);
        let nums_with_max_freq = 
            map.values().filter(|&&freq| freq == max_freq).count();

        nums_with_max_freq as i32 * max_freq
    }
}

fn main() {
    let nums = vec![1, 2, 2, 3, 1, 4];
    let result = Solution::max_frequency_elements(nums);
    println!("Resultado: {}", result);
}
