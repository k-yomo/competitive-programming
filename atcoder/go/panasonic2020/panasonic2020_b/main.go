package main

import "fmt"

func main() {
	var h, w int
	fmt.Scan(&h, &w)
	if h == 1 || w == 1 {
		fmt.Println(1)
		return
	}
	if h*w%2 == 0 {
		fmt.Println(h * w / 2)
	} else {
		fmt.Println(h*w/2 + 1)
	}
}
