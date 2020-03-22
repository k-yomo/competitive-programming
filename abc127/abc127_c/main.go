package main

import "fmt"

// https://atcoder.jp/contests/abc127/tasks/abc127_c
func main() {
	var cardNum, gateNum int
	fmt.Scan(&cardNum, &gateNum)

	curMin := 1
	curMax := cardNum

	for i := 0; i < gateNum; i++ {
		var min, max int
		fmt.Scan(&min, &max)
		if min > curMin {
			curMin = min
		}

		if max < curMax {
			curMax = max
		}
	}

	masterCardNum := curMax - curMin + 1
	if masterCardNum < 0 {
		masterCardNum = 0
	}
	fmt.Println(masterCardNum)
}
