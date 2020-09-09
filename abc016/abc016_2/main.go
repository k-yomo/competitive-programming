package main

import "fmt"

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	sum := a+b
	diff := a-b
	switch {
	case sum == c && diff == c:
		fmt.Println("?")
	case sum == c:
		fmt.Println("+")
	case diff == c:
		fmt.Println("-")
	default:
		fmt.Println("!")
	}
}
