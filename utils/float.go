package utils

import (
	"fmt"
	"math"
)

func ScanFloats32(len int) (floats []float32) {
	var f float32
	for i := 0; i < len; i++ {
		fmt.Scan(&f)
		floats = append(floats, f)
	}
	return
}

func AbsFloat32(n1 float32, n2 float32) float32 {
	return float32(math.Abs(float64(n1 - n2)))
}

func ScanFloats64(len int) (floats []float64) {
	var f float64
	for i := 0; i < len; i++ {
		fmt.Scan(&f)
		floats = append(floats, f)
	}
	return
}
