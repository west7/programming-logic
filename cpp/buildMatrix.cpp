// 2392. Build a Matrix With Conditions
// link: https://leetcode.com/problems/build-a-matrix-with-conditions/description/?envType=daily-question&envId=2024-07-21

#include <iostream>
#include <vector>
#include <queue>

using namespace std;

class Solution
{
public:
    vector<int> topological_sort(int k, vector<vector<int>> &conditions)
    {
        vector<int> degree(k + 1);
        vector<vector<int>> graph (k + 1);

        for (auto &condition : conditions)
        {
            graph[condition[0]].push_back(condition[1]);
            degree[condition[1]]++;
        }

        queue<int> q;
        for (int i = 1; i <= k; ++i)
            if (degree[i] == 0)
                q.push(i);

        int cnt = 0;
        vector<int> order;
        while (!q.empty())
        {
            int cur = q.front();
            q.pop();
            cnt++;
            order.push_back(cur);

            for (int neighbor : graph[cur])
            {
                degree[neighbor]--;
                if (degree[neighbor] == 0)
                {
                    q.push(neighbor);
                }
            }
        }

        if (cnt == k)
            return order;
        return {};
    }

    vector<vector<int>> buildMatrix(int k, vector<vector<int>> &rowConditions, vector<vector<int>> &colConditions)
    {
        vector<int> rowOrder = topological_sort(k, rowConditions);
        vector<int> colOrder = topological_sort(k, colConditions);

        if (rowOrder.empty() or colOrder.empty())
            return {};

        vector<vector<int>> matrix(k, vector<int>(k, 0));
        vector<pair<int, int>> index(k);

        for (auto i = 0; i < k; ++i)
        {
            index[rowOrder[i] - 1].first = i;
            index[colOrder[i] - 1].second = i;
        }

        for (auto i = 0; i < k; ++i)
            matrix[index[i].first][index[i].second] = i + 1;

        return matrix;
    }
};

void printMatrix(const std::vector<std::vector<int>> &matrix)
{
    for (const auto &row : matrix)
    {
        for (int val : row)
        {
            std::cout << val << " ";
        }
        std::cout << std::endl;
    }
}

int main()
{
    Solution s;

    vector<vector<int>> rowConditions = {{1, 2}, {3, 2}};
    vector<vector<int>> colConditions = {{2, 1}, {3, 2}};

    printMatrix(s.buildMatrix(3, rowConditions, colConditions));

    return 0;
}