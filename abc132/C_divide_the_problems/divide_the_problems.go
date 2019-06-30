package main

import (
	"fmt"
	"sort"
)

func main()  {
	var n int
	fmt.Scan(&n)
	nums := ScanNums(n)
	sort.Ints(nums)
	half := len(nums) / 2
	fmt.Println(nums[half] - nums[half-1])
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
