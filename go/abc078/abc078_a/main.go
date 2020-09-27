package main

import "fmt"

func main() {
	var a, b string
	fmt.Scan(&a, &b)
	if a > b {
		fmt.Println(">")
	} else if a < b {
		fmt.Println("<")
	} else {
		fmt.Println("=")
	}
}
