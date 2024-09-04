// https://leetcode.com/problems/binary-search/description/
// 704. Binary Search
// Easy

// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

// You must write an algorithm with O(log n) runtime complexity.



// Example 1:

// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
// Example 2:

// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1


// Constraints:

// 1 <= nums.length <= 104
// -104 < nums[i], target < 104
// All the integers in nums are unique.
// nums is sorted in ascending order.

function search(nums: number[], target: number): number {
    let [l, r] = [0, nums.length - 1]
    while (l <= r) {
        const mid = (l + r) >> 1
        console.log(nums.slice(l, r), l, r, mid, "Mid number: ", nums[mid])

        if (target > nums[mid]) {
            l = mid + 1
        } else if (target < nums[mid])
            r = mid - 1
        else return mid
    }

    return -1
};

console.clear()
console.log(search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7))
console.log(search([1, 2, 3, 4, 5, 6, 7, 9, 10], 8))
