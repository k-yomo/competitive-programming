package main

import "fmt"

func main() {
	var a, b string
	fmt.Scan(&a, &b)
	fmt.Println(fmt.Sprintf("A%sC", string(b[0])))
}
