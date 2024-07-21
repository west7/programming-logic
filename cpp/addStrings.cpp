// 415. Add Strings
// link: https://leetcode.com/problems/add-strings/description/

#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

class Solution
{
public:
    string addStrings(string num1, string num2)
    {

        int i = num1.size() - 1, j = num2.size() - 1;
        int carry = 0;
        string result; 

        while (i >= 0 or j >= 0 or carry) {

            int d1 = (i >= 0) ? num1[i] - '0' : 0;
            int d2 = (j >= 0) ? num2[j] - '0' : 0; 
            
            int sum = d1 + d2 + carry;

            carry = sum / 10;
            result.push_back((sum % 10) + '0');

            i--, j--;
        }

        reverse(result.begin(), result.end());
    
        return result;
    }
};

int main()
{
    Solution s;
    string num1 = "123", num2 = "456";
    cout << s.addStrings(num1, num2) << endl;

    return 0;
}