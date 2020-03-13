package main

import (
	"fmt"
)

func main() {
	var x, t int
	fmt.Scan(&x, &t)
	if x-t > 0 {
		fmt.Println(x - t)
	} else {
		fmt.Println(0)
	}
}
