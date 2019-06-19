package main

import "fmt"

func main() {
	var k int
	fmt.Scan(&k)
	antennas := ScanNums(5)

	anyUnconnectable := false
	for _, length := range antennas[1:] {
		if length - antennas[0] > k {
			anyUnconnectable = true
		}
	}

	if anyUnconnectable {
		fmt.Println(":(")
	} else {
		fmt.Println("Yay!")
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
