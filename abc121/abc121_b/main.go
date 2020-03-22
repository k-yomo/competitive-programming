package main

import "fmt"

func main()  {
	var n, m, c int
	fmt.Scan(&n, &m, &c)
	nums := ScanNums(m)
	ans := 0
	for i := 0; i < n; i++ {
		sourceCodes := ScanNums(m)
		sum := c
		for j, n := range sourceCodes{
			sum += n * nums[j]
		}
		if sum > 0 {
			ans++
		}
	}
	fmt.Println(ans)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}

