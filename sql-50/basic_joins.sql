-- SQL 50 Leetcode Study Plan - "Basic joins" session.   
-- https://leetcode.com/studyplan/top-sql-50/

-- 1378. Replace Employee ID With The Unique Identifier
-- https://leetcode.com/problems/replace-employee-id-with-the-unique-identifier/description/?envType=study-plan-v2&envId=top-sql-50

SELECT 
    IFNULL(EmployeeUNI.unique_id, NULL) as unique_id, Employees.name
FROM 
    Employees
LEFT JOIN
    EmployeeUNI ON Employees.id = EmployeeUNI.id;