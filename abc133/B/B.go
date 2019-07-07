package main

import (
	"fmt"
	"math"
)

func main() {
	var n, d int
	fmt.Scan(&n, &d)
	cordinates := ScanCordinates(n, d)
	ans := 0
	for i := 0; i < len(cordinates)-1; i++ {
		for j := i + 1; j < len(cordinates); j++ {
			n := 0.0
			for k := 0; k < d; k++ {
				n += math.Pow(math.Abs(float64(cordinates[i][k]-cordinates[j][k])), 2)
			}
			if math.Sqrt(n) == math.Trunc(math.Sqrt(n)) {
				ans++
			}
		}
	}
	fmt.Println(ans)
}

func ScanCordinates(n int, d int) (cordinates [][]int) {
	for i := 0; i < n; i++ {
		cordinates = append(cordinates, ScanNums(d))
	}
	return cordinates
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
