package main

import "fmt"

func main() {
	var k, a, b int
	fmt.Scan(&k, &a, &b)
	for i := 1; i < 1000; i++ {
		if k*i >= a && k*i <= b {
			fmt.Println("OK")
			return
		}
		if k*i > b {
			fmt.Println("NG")
			return
		}
	}
}
