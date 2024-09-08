#![allow(unused)]
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut u_index = 0;

        for i in 0..nums.len() {
            if nums[i] != nums[u_index] {
                u_index += 1;
                nums[u_index] = nums[i];
            }
        }

        u_index as i32 + 1
    }
}

fn main() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let k = Solution::remove_duplicates(&mut nums);
    println!("{:?}", &nums[..k as usize]);
}