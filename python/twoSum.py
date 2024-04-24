# 1. Two Sum
# link: https://leetcode.com/problems/two-sum/description/

class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        num_dict = {}
        for i, n in enumerate(nums):
            complement = target - n
            if complement in num_dict:
                return [num_dict[complement], i]
            num_dict[complement] = n