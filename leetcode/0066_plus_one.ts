// EASY
// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

// Increment the large integer by one and return the resulting array of digits.



// Example 1:

// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Incrementing by one gives 123 + 1 = 124.
// Thus, the result should be [1,2,4].
// Example 2:

// Input: digits = [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.
// Incrementing by one gives 4321 + 1 = 4322.
// Thus, the result should be [4,3,2,2].
// Example 3:

// Input: digits = [9]
// Output: [1,0]
// Explanation: The array represents the integer 9.
// Incrementing by one gives 9 + 1 = 10.
// Thus, the result should be [1,0].


// Constraints:

// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits does not contain any leading 0's.

function plusOne_v1(digits: number[]): number[] {
    let l = 0
    digits = digits.reverse()
    while (l < digits.length) {
        digits[l]++
        if (digits[l] >= 10) {
            digits[l] = 0
            if (!digits[l + 1]) {
                digits.push(1)
                break
            }
        } else break
        l++
    }
    return digits.reverse()
};

function plusOne_v2(digits: number[]): number[] {
    return (BigInt(digits.join('')) + BigInt(1)).toString().split('').map(Number)
};

function plusOne_v3(digits: number[]): number[] {
    let l = digits.length - 1
    while (l >= 0) {
        if (digits[l] < 9) {
            digits[l]++
            return digits
        }
        digits[l] = 0
        l--
    }
    digits = new Array(digits.length + 1).fill(0)
    digits[0] = 1
    return digits
};

function plusOne_v4(digits: number[]): number[] {
    let l = digits.length - 1
    while (l >= 0) {
        if (digits[l] < 9) {
            digits[l]++
            return digits
        }
        digits[l] = 0
        l--
    }
    digits.unshift(1)
    return digits
};

console.log(plusOne_v3([6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,3])) // ... 5,4,4
console.log(plusOne_v3([6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,9])) // ... 5,5,0
console.log(plusOne_v3([1, 9])) // 20
console.log(plusOne_v3([1, 0])) // 11
console.log(plusOne_v3([9, 9, 9, 9, 9, 9, 9, 9, 9])) // [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
console.log(plusOne_v3([9])) // 10
