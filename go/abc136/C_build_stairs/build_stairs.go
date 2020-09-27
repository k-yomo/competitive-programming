package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	nums := ScanNums(n)
	isNonDecreasing := true

	for i := n - 1; i > 0; i-- {
		if nums[i-1] <= nums[i] {
			continue
		} else if nums[i-1]-1 <= nums[i] {
			nums[i-1] = nums[i-1] - 1
		} else {
			isNonDecreasing = false
			break
		}
	}

	if isNonDecreasing {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
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
