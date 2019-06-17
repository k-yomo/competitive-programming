package main

import (
	"fmt"
)

// https://atcoder.jp/contests/abc130/tasks/abc130_d
func main() {
	var n, k int
	fmt.Scan(&n, &k)
	nums := ScanNums(n)

	var sum, count, right int
	for left := 0; left < n; left++ {
		for ; right < n && sum < k; right++ {
			sum += nums[right]
		}
		if sum >= k {
			count += n - right + 1
		}
		sum -= nums[left]
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
