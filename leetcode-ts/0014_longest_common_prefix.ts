// 14. Longest Common Prefix
// Easy
// https://leetcode.com/problems/longest-common-prefix/description/

// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".



// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.


// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.


function longestCommonPrefix_v2(strs: string[]): string {
    if (strs.length === 1) return strs[0]

    // keep looping on all string 
    // verify one character at time j
    // if all strings match at character j then concat to the prefix 
    // otherwise break the inner and outer loop
    let prefix = ""
    while (true) {
        if (strs[0][prefix.length] === undefined) return prefix
        prefix = prefix + strs[0][prefix.length]
        let i = 1;
        while (i < strs.length) {
            if (strs[i].length === 0) break
            if (strs[i][prefix.length - 1] !== prefix[prefix.length - 1]) break
            i++
        }
        if (i < strs.length) {
            prefix = prefix.substring(0, prefix.length - 1);
            break
        }

    }
    return prefix
};
function longestCommonPrefix(strs: string[]): string {
    const v = strs.sort()
    const f = v[0] // first
    const l = v[v.length - 1] // length
    let i = 0;
    let prefix = ""
    while (i < Math.min(f.length, l.length)) {
        if (f[i] !== l[i]) return prefix
        prefix += f[i]
        i++
    }
    return prefix
};


console.clear()
console.log(1, longestCommonPrefix(["flower", "flow", "flight"]))
console.log(2, longestCommonPrefix([""]))
console.log(3, longestCommonPrefix(["", ""]))
console.log(4, longestCommonPrefix(["car", "car", "carn"]))
console.log(5, longestCommonPrefix(["car", "car", "car"]))
console.log(6, longestCommonPrefix(["abc", "add", "adqwg", "abcasfdgsdgsdg"]))
