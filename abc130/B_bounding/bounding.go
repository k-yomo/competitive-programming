package main

import "fmt"

// https://atcoder.jp/contests/abc130/tasks/abc130_b
func main()  {
	var n, x int
	fmt.Scan(&n, &x)
	nums := ScanNums(n)
	boundNum := 1
	curPos := 0
	for i := 0; i < len(nums); i++ {
		curPos =  curPos + nums[i]
		if curPos > x {
			break
		}
		boundNum++
	}
	fmt.Println(boundNum)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
