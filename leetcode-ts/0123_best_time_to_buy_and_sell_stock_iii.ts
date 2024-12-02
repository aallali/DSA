// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/description/
// 123. Best Time to Buy and Sell Stock III
// Hard
// You are given an array prices where prices[i] is the price of a given stock on the ith day.

// Find the maximum profit you can achieve. You may complete at most two transactions.

// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

// Example 1:

// Input: prices = [3,3,5,0,0,3,1,4]
// Output: 6
// Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
// Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
// Example 2:

// Input: prices = [1,2,3,4,5]
// Output: 4
// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
// Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
// Example 3:

// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.

// Constraints:

// 1 <= prices.length <= 105
// 0 <= prices[i] <= 105

function maxProfit(prices: number[]): number {
  let [l, r, minL, maxR, profit] = [0, prices.length, prices[0], prices[0], 0];
  const profits: number[] = [];
  while (l < r) {
    maxR = Math.max(prices[l], maxR);
    if (prices[l] < minL) {
      minL = prices[l];
      maxR = minL;
      profit = 0;
    }
    let prof = maxR - minL
    if (prof && prof !== profit) {
      profit = prof
      profits.push(prof)
      minL = prices[l];
      maxR = minL;
      profit = 0;
    }
    l++;
  }
  if ([...(new Set(profits))].length === 1) 
      return profits[0] * profits.length
  let maxProfit = 0;
  l = 0, r = profits.length - 1 
  while (l < r) {
    maxProfit = Math.max(maxProfit, profits[l]+profits[l+1])
    l++
  }
 
  return maxProfit;
}

console.clear();

// console.log(maxProfit([1, 2, 3, 4, 5])); // 4
// console.log(maxProfit([3, 3, 5, 0, 0, 3, 1, 4])); // 6
// console.log(maxProfit([7,6,4,3,1])); // 0
console.log(maxProfit([6,1,3,2,4,7])); // 7
