// 43. Multipĺy Strings
// link: https://leetcode.com/problems/multiply-strings/

#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution
{
public:
    string multiply(string num1, string num2)
    {
        if (num1 == "0" or num2 == "0")
            return "0";

        int l1 = num1.size();
        int l2 = num2.size();

        vector<int> result(l1 + l2, 0);

        for (int i = l1 - 1; i >= 0; --i)
        {
            for (int j = l2 - 1; j >= 0; --j)
            {
                int mul = (num1[i] - '0') * (num2[j] - '0'); // subtrair o char '0' faz o casting da string para int
                int sum = mul + result[i + j + 1];

                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        string product;
        for (int num : result)
        {
            if (!(product.empty() and num == 0))
                product.push_back(num + '0');
        }

        return product.empty() ? "0" : product;
    }
};

/*
Notas:

Nas linhas 27, 29 e 30, o que acontece é um acúmulo de valores ao realizar sum = mul + result[i + j + 1], adicionar a posição
do result é importante para não sobrepor possíveis carrys de multiplicações anteriores, por exemplo:

num1 = "3"
num2 = "6"

Em um primeiro momento, result[i + j + 1] = 0, então mul = 18 e sum = 18, portanto o vetor result ficará assim:

result = [0, 0]

result [i + j + 1] = sum % 10; // indíce = 0 + 0 + 1 = 1, 18 % 10 = 8
--> [0, 8]

result [i + j] = sum / 10; // uma posição anterior, 18 / 10 = 1
--> [1, 8]

Formando assim o resultado exato, 3 * 6 = 18.

Agora para:

num1 = "12"
num2 = "5"

result = [0, 0, 0]

1ª iteração
i = 1, j = 0
mul = 2 * 5 = 10, sum = 10 + 0 = 10

result [i + j + 1] = 10 % 10;
--> [0, 0, 0]

result [i + j] += 10 / 10
--> [0, 1, 0]
---

2ª iteração
i = 0, j = 0
mul = 1 * 5 = 5, sum = 5 + 1 = 6

result [i + j + 1] = 6 % 10 = 6;
--> [0, 6, 0]

result [i + j] = 6 / 10 = 0;
--> [0, 6, 0]

Assim, 12 * 5 = 60;
*/

int main() {
    Solution s;
    std::cout << "A multiplicação de 12 e 5 é: " << s.multiply("12", "5") << endl;

    return 0;
}