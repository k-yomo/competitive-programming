package main

import (
	"fmt"
)

func main() {
	var n int
	var s string
	fmt.Scan(&n, &s)
	passwordMap := map[string]bool{}
	var count int
	leftNumMap := map[string]bool{}
	for i := 0; i < n-2; i++ {
		if len(leftNumMap) == 10 {
			break
		}

		leftNum := s[i : i+1]
		if leftNumMap[leftNum] {
			continue
		}
		leftNumMap[leftNum] = true

		middleNumMap := map[string]bool{}
		for j := i + 1; j < n-1; j++ {
			if len(middleNumMap) == 10 {
				break
			}

			middleNum := s[j : j+1]
			if middleNumMap[middleNum] {
				continue
			}
			middleNumMap[middleNum] = true

			rightNumMap := map[string]bool{}
			for k := j + 1; k < n; k++ {
				if len(rightNumMap) == 10 {
					break
				}

				rightNum := s[k : k+1]
				password := leftNum + middleNum + rightNum
				if passwordMap[password] {
					continue
				}

				rightNumMap[rightNum] = true
				passwordMap[password] = true
				count++
			}
		}
	}
	fmt.Println(count)
}
