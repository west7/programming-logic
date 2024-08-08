// 13. Roman to integer
// link: https://leetcode.com/problems/roman-to-integer/description/

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let mut symbols: HashMap<char, i32> = HashMap::new();
        symbols.insert('I', 1);
        symbols.insert('V', 5);
        symbols.insert('X', 10);
        symbols.insert('L', 50);
        symbols.insert('C', 100);
        symbols.insert('D', 500);
        symbols.insert('M', 1000);

        let mut ans = 0;
        let mut prev_val = 0;

        for letter in s.chars().rev() {
            let val = symbols[&letter];

            if prev_val > val {
                ans -= val;
            } else {
                ans += val;
            }
            prev_val = val;
        }

        ans
    }
}

fn main() {
    let s = "III";
    let res = Solution::roman_to_int(s.to_string());
    assert_eq!(res, 3);

    let s: &str = "LVIII";
    let res = Solution::roman_to_int(s.to_string());
    assert_eq!(res, 58);

    let s = "MCMXCIV";
    let res = Solution::roman_to_int(s.to_string());
    assert_eq!(res, 1994);
}
