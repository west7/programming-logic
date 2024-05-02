# 3. Longest Substring without repeating characters
# link: https://leetcode.com/problems/longest-substring-without-repeating-characters/

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        letter_dict = {}
        start = 0
        max_len = 0

        for i, letter in enumerate(s):
            if letter in letter_dict and letter_dict[letter] >= start:
                start = letter_dict[letter] + 1
            letter_dict[letter] = i
            max_len = max(max_len, i - start + 1)

        return max_len