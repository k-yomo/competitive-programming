package main

import (
	"fmt"
	"math"
)

// 答えは下記のいずれか
// 最小値にいく前にカウントが尽きる
// 最小値
// 最小値の１個逆側(回数的に最小値にできない)
func main() {
	var x, k, d int
	fmt.Scan(&x, &k, &d)
	absX := AbsDiff(x, 0)
	var min int
	var count int
	if absX - d*k > 0 {
		fmt.Println(absX - d*k)
		return
	}
	if d < absX {
		min = absX % d
		count = absX / d
		if count >= k {
			min = absX - d*k
		}
	} else {
		min = absX - d
		count = 1
	}
	if k > count && (k-count)%2 != 0 {
		if min > 0 {
			min = min - d
		} else {
			min = min + d
		}
	}
	fmt.Println(AbsDiff(min, 0))
}

func AbsDiff(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}
