package main

import (
	"fmt"
	"math"
)

func main()  {
	var a, b, c, d int
	fmt.Scan(&a, &b, &c, &d)
	if AbsInt(a, c) <= d || AbsInt(a, b) <= d && AbsInt(b, c) <= d {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}

func AbsInt(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}
