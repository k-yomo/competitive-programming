package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var n int
	fmt.Scan(&n)
	boxes := ScanNums(n)
	var newBoxes = make([]string, 0)
	noGoodWayExist := false

	for i, ballNum := range boxes {
		var ballTotal int
		for j, innerBallNum := range boxes {
			if (j+1)%(i+1) == 0 {
				ballTotal += innerBallNum
			}
		}
		if ballTotal%2 != ballNum {
			noGoodWayExist = true
			break
		}
		if ballTotal%2 != 0 {
			newBoxes = append(newBoxes, strconv.Itoa(i+1))
		}
	}

	if noGoodWayExist {
		fmt.Println(-1)
	} else {
		fmt.Println(len(newBoxes))
		if len(newBoxes) > 0 {
			fmt.Println(strings.Join(newBoxes, " "))
		}
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
