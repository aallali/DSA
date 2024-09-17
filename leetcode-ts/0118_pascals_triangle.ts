// https://leetcode.com/problems/pascals-triangle/description/
// Easy

// Given an integer numRows, return the first numRows of Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
// https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif
// Example 1:

// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:

// Input: numRows = 1
// Output: [[1]]
 

// Constraints:
// 1 <= numRows <= 30

function generate(numRows: number): number[][] {
    const triangle: number[][] = []
    for (let i = 0; i < numRows; i++)
        triangle.push(new Array(i + 1).fill(1))

    for (let i = 0; i < triangle.length - 1; i++) {
        const row = triangle[i]

        for (let j = 0; j < row.length; j++) {
            if (j + 1 < row.length)
                triangle[i + 1][j + 1] = row[j] + row[j + 1]
        }
    }
    return triangle
};
console.clear()
console.log(generate(5))