package main

import "fmt"

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	var a, b string
	fmt.Scan(&a, &b)
	if reverse(a) == b {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
