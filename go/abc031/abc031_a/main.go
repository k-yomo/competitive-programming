package main

import "fmt"

func main() {
	var a, d int
	fmt.Scan(&a, &d)

	if (a+1)*d > a*(d+1) {
		fmt.Println((a + 1) * d)
	} else {
		fmt.Println(a * (d + 1))
	}
}
