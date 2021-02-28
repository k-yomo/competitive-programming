package main

import (
	"fmt"
	"math"
	"math/bits"
	"strconv"
)

func main() {
	var n int
	var x string
	fmt.Scan(&n, &x)

	xNum, _ := strconv.ParseInt(x, 2, 64)
	bitModCountMap := map[int]int{}
	xor := int(math.Pow(float64(2), float64(n-1)))
	for i := 0; i < n; i++ {
		if i > 0 {
			xor = xor / 2
		}
		num := int(xNum) ^ xor
		tempNum := num
		var count int
		for tempNum != 0 {
			if c, ok := bitModCountMap[tempNum]; ok {
				count += c
				break
			}
			tempNum = tempNum % bits.OnesCount(uint(tempNum))
			count++
		}
		bitModCountMap[num] = count
		fmt.Println(count)
	}
}
