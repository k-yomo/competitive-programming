package main

import "fmt"

func main() {
	var x, a, b int
	fmt.Scan(&x, &a, &b)
	if -a+b > x {
		fmt.Println("dangerous")
	} else if -a+b <= 0 {
		fmt.Println("delicious")
	} else {
		fmt.Println("safe")
	}
}
