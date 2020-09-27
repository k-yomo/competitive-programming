package main

import (
	"fmt"
	"math"
)

func main() {
	var n, a, b int
	fmt.Scan(&n, &a, &b)

	var position int
	for i := 0; i < n; i++ {
		var s string
		var d int
		fmt.Scan(&s, &d)
		var meter int
		switch {
		case d < a:
			meter = a
		case a <= d && d <= b:
			meter = d
		default:
			meter = b
		}
		if s == "East" {
			meter *= -1
		}
		position += meter
	}

	switch {
	case position < 0:
		fmt.Println("East", int(math.Abs(float64(position))))
	case position > 0:
		fmt.Println("West", position)
	default:
		fmt.Println(0)
	}
}
