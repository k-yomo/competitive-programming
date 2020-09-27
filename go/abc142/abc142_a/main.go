package main

import "fmt"

func main() {
	var n float64
	fmt.Scan(&n)
	fmt.Println(float64((int(n)+1)/2) / n)
}
