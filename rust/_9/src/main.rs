// 9. Palindrome number
// link: https://leetcode.com/problems/palindrome-number/description/

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut original = x;
        let mut rev = 0;

        while original != 0 {
            let digit = original % 10;
            rev = rev * 10 + digit;
            original /= 10;
        }
        x == rev
    }
}

fn main() {
    let x: i32 = 121;
    let res = Solution::is_palindrome(x);
    assert_eq!(res, true);

    let x: i32 = -121;
    let res = Solution::is_palindrome(x);
    assert_eq!(res, false);

    let x: i32 = 10;
    let res = Solution::is_palindrome(x);
    assert_eq!(res, false);
}
