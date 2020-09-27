package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)
	bNums := ScanNums(n-1)
	aNums := make([]int, n)
	aNums[n-1] = bNums[len(bNums)-1]
	for i := n-2; i >= 0; i-- {
		var max int
		if i-1 < 0 || bNums[i] < bNums[i-1] {
			max = bNums[i]
		} else {
			max = bNums[i-1]
		}
		aNums[i] = max
	}

	maxSum := 0
	for _, num := range aNums {
		maxSum += num
	}
	fmt.Println(maxSum)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
