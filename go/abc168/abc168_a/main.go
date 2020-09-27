package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)
	firstNum := n % 10
	if firstNum == 3 {
		fmt.Println("bon")
	} else if ContainsInt([]int{0, 1, 6, 8}, firstNum) {
		fmt.Println("pon")
	} else {
		fmt.Println("hon")
	}
}

func ContainsInt(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}
