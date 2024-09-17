function searchMatrix(matrix: number[][], target: number): boolean {
    return search(matrix.flat(), target)
};

function search(nums: number[], target: number): boolean {
    let [l, r] = [0, nums.length - 1]
    while (l <= r) {
        const mid = (l + r) >> 1 // Math.floor((l+r) / 2)
        if (target === nums[mid]) return true
        else if (target > nums[mid]) l = mid + 1
        else r = mid - 1
    }

    return false;
};
console.clear()

console.log(searchMatrix([[1], [2]], 0))
console.log(searchMatrix([[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]], 3))
