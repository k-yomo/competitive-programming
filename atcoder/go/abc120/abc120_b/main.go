package main

import "fmt"

func main() {
	var a, b, k int
	fmt.Scan(&a, &b, &k)
	var max int
	if a > b {
		max = a
	} else {
		max = b
	}

	nums := make([]int, 0)
	for i := 1; i <= max; i++ {
		if a%i == 0 && b%i == 0 {
			nums = append(nums, i)
		}
	}
	fmt.Println(nums[len(nums) - k])
}
