package main

import (
	"fmt"
	"math"
)

func main() {
	var p float64
	fmt.Scan(&p)

	yearAfter := TernarySearch(0, 10000.0, func(yearAfter float64) float64 {
		return calcTime(yearAfter, p)
	})
	fmt.Println(calcTime(yearAfter, p))
}

func calcTime(yearAfter, p float64) float64 {
	return yearAfter + p / math.Pow(2, yearAfter/ 1.5)
}

func TernarySearch(l, r float64, calcTime func(float64) float64) float64 {
	for math.Abs(l-r) > 0.00000001 {
		ml := l + (r-l) / 3
		mr := r - (r-l) / 3
		if calcTime(ml) > calcTime(mr) {
			l = ml
		} else {
			r = mr
		}
	}
	return l
}
