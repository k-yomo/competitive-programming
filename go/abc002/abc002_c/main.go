package main

import (
	"fmt"
	"math"
)

func main() {
	var xa, ya, xb, yb, xc, yc float64
	fmt.Scan(&xa, &ya, &xb, &yb, &xc, &yc)
	fmt.Println(math.Abs((xa-xc)*(yb-yc)-(xb-xc)*(ya-yc)) / 2)
}
