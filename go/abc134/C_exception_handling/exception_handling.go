package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)
	nums := ScanNums(n)
	sortedNums := make([]int, n)
	copy(sortedNums, nums)
	sort.Ints(sortedNums)
	max := sortedNums[n-1]
	secondMax := sortedNums[n-2]
	for _, num := range nums {
		if num != max {
			fmt.Println(max)
		} else {
			fmt.Println(secondMax)
		}
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
