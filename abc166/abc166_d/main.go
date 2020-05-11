package main

import (
	"fmt"
)

func main() {
	var x int
	fmt.Scan(&x)
	for i := -1000;  i < 1001; i++ {
		for j := -1000;  j < 1001; j++ {
			if i*i*i*i*i - j*j*j*j*j == x {
				fmt.Println(i, j)
				return
			}
		}
	}
}
