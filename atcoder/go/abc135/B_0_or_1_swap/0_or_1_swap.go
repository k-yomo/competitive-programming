package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	var incorrectOrderNum int
	fmt.Scan(&n)
	nums := ScanNums(n)
	sortedNums := make([]int, n)
	copy(sortedNums, nums)
	sort.Ints(sortedNums)

	for i := 0; i < n; i++ {
		if nums[i] != sortedNums[i] {
			incorrectOrderNum++
		}
	}

	if incorrectOrderNum == 0 || incorrectOrderNum == 2 {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
