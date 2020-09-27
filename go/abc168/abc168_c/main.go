package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b, h, m float64
	fmt.Scan(&a, &b, &h, &m)
	hDeg := h*30 + m*0.5
	mDeg := m * 6
	ang := math.Abs(hDeg - mDeg)
	fmt.Println(math.Sqrt(a*a + b*b - 2*a*b*math.Cos(ang * math.Pi / 180)))
}
