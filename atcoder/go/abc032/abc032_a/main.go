package main

import "fmt"

func main() {
	var a, b, n int
	fmt.Scan(&a, &b, &n)
	for  {
		if n%a == 0 && n%b == 0 {
			fmt.Println(n)
			return
		}
		n++
	}
}
