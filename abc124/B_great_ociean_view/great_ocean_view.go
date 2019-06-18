package main

import "fmt"

// https://atcoder.jp/contests/abc124/tasks/abc124_b
func main()  {
	var n int
	fmt.Scan(&n)
	nums := ScanNums(n)
	max := nums[0]
	count := 1
	for i := 1; i < n; i++ {
		if nums[i] >= max {
			count++
			max = nums[i]
		}
	}
	fmt.Println(count)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
