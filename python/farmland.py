# 1992. Find all groups of farmlands
# link: https://leetcode.com/problems/find-all-groups-of-farmland/description/?envType=daily-question&envId=2024-04-20

class Solution:
    def findFarmland(self, land: list[list[int]]) -> list[list[int]]:
        
        if not land:
            return []

        rows = len(land)
        cols = len(land[0])

        submatrix_coordinates = []

        def expand_submatrix(start_row, start_col):
            end_row = start_row
            end_col = start_col

            while end_row < rows and land[end_row][start_col] == 1:
                end_row += 1

            while end_col < cols and land[start_row][end_col] == 1:
                end_col += 1

            for r in range(start_row, end_row):
                for c in range(start_col, end_col):
                    land[r][c] = 0

            return end_row - 1, end_col - 1
        

        for i in range(rows):
            for j in range(cols):
                if land[i][j] == 1:
                    end_row, end_col = expand_submatrix(i, j)
                    submatrix_coordinates.append([i, j, end_row, end_col])

        return submatrix_coordinates