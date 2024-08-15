// 27. Remove Element
// link: https://leetcode.com/problems/remove-element/description/?envType=study-plan-v2&envId=top-interview-150

#![allow(unused)]

use std::ops::Rem;

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut removes: usize = 0;

        for i in (0..nums.len()).rev() {
            if nums[i] == val {
                nums.remove(i);  // Expensive
                removes += 1;
            }
        }
        nums.len() as i32
    }
    // Complexity: O(n²)

    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write_index = 0;

        for read_index in 0..nums.len() {
            if nums[read_index] != val {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }
        write_index as i32 
    }
    // Complexity: O(n)
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let expected_nums = vec![2, 2, 3, 3];

    let k = Solution::remove_element2(&mut nums, val);
    // assert_eq!(nums, expected_nums);
    println!("k: {k}, {nums:?}");

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let expected_nums = vec![0, 1, 4, 0, 3, 2, 2, 2];

    let k = Solution::remove_element2(&mut nums, val);
    // assert_eq!(nums, expected_nums);
    println!("k: {k}, {nums:?}");
}
