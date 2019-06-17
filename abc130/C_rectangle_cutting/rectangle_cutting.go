package main

import "fmt"

// https://atcoder.jp/contests/abc130/tasks/abc130_c
func main() {
	var w, h, x, y float64
	fmt.Scan(&w, &h, &x, &y)
	max := w * h / 2.0
	multipleWay := 0
	if x == w/2 && y == h/2 {
		multipleWay = 1
	}
	fmt.Printf("%f %d", max, multipleWay)
}

