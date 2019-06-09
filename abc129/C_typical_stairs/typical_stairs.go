package main

import (
	"fmt"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	brokenStairs := ScanNums(m)

	brokenStairsCount := len(brokenStairs)
	brokenStairsMapping := map[int]bool{}
	for i := 0; i < brokenStairsCount; i++ {
		stairNum := brokenStairs[i]
		brokenStairsMapping[stairNum] = true
		if i+1 == brokenStairsCount {
			continue
		}
		if stairNum+1 == brokenStairs[i+1] {
			fmt.Println(0)
			return
		}
	}

	dp := make([]int, n+1)
	for i := 0; i <= n; i++ {
		if brokenStairsMapping[i] {
			continue
		}
		if i < 2 {
			dp[i] = 1
		} else {
			dp[i] = (dp[i-1] + dp[i-2]) % 1000000007
		}
	}
	fmt.Println(dp[n])
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
