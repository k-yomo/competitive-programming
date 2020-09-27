package main

import "fmt"

func main() {
	var c string
	fmt.Scan(&c)
	fmt.Printf("%c\n", []rune(c)[0]+1)
}
